// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::{sync::Arc, thread, time::Duration};
use tokio::{sync::broadcast, sync::Mutex, time};
use winapi::um::winuser::GetKeyState;
use enigo::*;

struct AsyncSender {
    inner: Mutex<broadcast::Sender<i32>>,
    vec: Arc<Mutex<Vec<Event>>>,
}

fn send(key: Key) {
    let delay = time::Duration::from_millis(20);
    
    let mut engio = Enigo::new();
    engio.key_click(key);

    thread::sleep(delay);
}

#[derive(Debug, Clone, Copy)]
struct Event {
    key: char,
    interval: i32,
}

impl PartialEq for Event {
    fn eq(&self, other: &Self) -> bool {
        self.key == other.key
    }
}

#[tauri::command]
async fn create_event(
    key: char,
    interval: i32,
    state: tauri::State<'_, AsyncSender>,
) -> Result<(), ()> {
    let event_list = &mut state.vec.lock().await;
    let e = Event { key, interval };

    dbg!(&e);

    if event_list.contains(&e) {
        for i in 0..event_list.len() {
            if event_list[i] == e {
                event_list[i] = e;
                break;
            }
        }
    } else {
        event_list.push(e);
    }

    Ok(())
}

#[tauri::command]
async fn delete_event(
    key: char,
    interval: i32,
    state: tauri::State<'_, AsyncSender>,
) -> Result<(), ()> {
    let event_list = &mut state.vec.lock().await;
    let e = Event { key, interval };

    let index = event_list.iter().position(|evt| evt.key == e.key).unwrap();
    event_list.remove(index);

    Ok(())
}

#[tauri::command]
async fn clear_events(state: tauri::State<'_, AsyncSender>) -> Result<(), ()> {
    let event_list = &mut state.vec.lock().await;

    event_list.clear();

    Ok(())
}

#[tokio::main]
async fn main() {
    tauri::async_runtime::set(tokio::runtime::Handle::current());

    let (tx, mut rx) = broadcast::channel(64);
    let event_list: Arc<Mutex<Vec<Event>>> = Arc::new(Mutex::new(Vec::new()));
    let event_list_rx: Arc<Mutex<Vec<Event>>> = event_list.clone();

    tauri::Builder::default()
        .manage(AsyncSender {
            inner: Mutex::new(tx.clone()),
            vec: event_list,
        })
        .setup(|_| {
            let toggle_tx = tx.clone();

            tauri::async_runtime::spawn_blocking(move || {
                let mut active = false;
                loop {
                    unsafe {
                        // Z
                        if GetKeyState(90).is_negative() {
                            active = !active;

                            toggle_tx.send(active as i32).unwrap();
                            thread::sleep(Duration::from_millis(200));
                        }
                    }
                }
            });

            tauri::async_runtime::spawn(async move {
                loop {
                    if rx.recv().await.unwrap() != 1 {
                        continue;
                    }

                    println!("test");

                    for e in event_list_rx.lock().await.iter() {
                        let mut rx_clone = tx.subscribe();
                        let key = e.key;
                        let intv = e.interval;

                        tokio::spawn(async move {
                            let mut interval = time::interval(Duration::from_millis(intv as u64));

                            while let Ok(_) = rx_clone.try_recv() {}

                            loop {
                                interval.tick().await;

                                match rx_clone.try_recv() {
                                    Err(broadcast::error::TryRecvError::Empty) => {}
                                    _ => break,
                                }

                                // let key_enum = match key {
                                //     'a' => Key::KeyA,
                                //     'b' => Key::KeyB,
                                //     'c' => Key::KeyC,
                                //     'd' => Key::KeyD,
                                //     'e' => Key::KeyE,
                                //     'f' => Key::KeyF,
                                //     'g' => Key::KeyG,
                                //     'h' => Key::KeyH,
                                //     'i' => Key::KeyI,
                                //     'j' => Key::KeyJ,
                                //     'k' => Key::KeyK,
                                //     'l' => Key::KeyL,
                                //     'm' => Key::KeyM,
                                //     'n' => Key::KeyN,
                                //     'o' => Key::KeyO,
                                //     'p' => Key::KeyP,
                                //     'q' => Key::KeyQ,
                                //     'r' => Key::KeyR,
                                //     's' => Key::KeyS,
                                //     't' => Key::KeyT,
                                //     'u' => Key::KeyU,
                                //     'v' => Key::KeyV,
                                //     'w' => Key::KeyW,
                                //     'x' => Key::KeyX,
                                //     'y' => Key::KeyY,
                                //     'z' => Key::KeyZ,
                                //     '0' => Key::Num0,
                                //     '1' => Key::Num1,
                                //     '2' => Key::Num2,
                                //     '3' => Key::Num3,
                                //     '4' => Key::Num4,
                                //     '5' => Key::Num5,
                                //     '6' => Key::Num6,
                                //     '7' => Key::Num7,
                                //     '8' => Key::Num8,
                                //     '9' => Key::Num9,
                                //     ' ' => Key::Space,
                                //     _ => Key::Space,
                                // };

                                send(Key::Layout(key));
                            }
                        });
                    }
                }
            });

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            create_event,
            delete_event,
            clear_events
        ])
        .run(tauri::generate_context!())
        .expect("Tauri Application Error");
}
