// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use enigo::*;
use std::{sync::Arc, time::Duration};
use tokio::{sync::broadcast, sync::Mutex, time};

struct AsyncSender {
    inner: Mutex<broadcast::Sender<i32>>,
    vec: Arc<Mutex<Vec<Event>>>,
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

    let index = event_list.iter().position(|evt| *evt == e).unwrap();
    event_list.remove(index);

    Ok(())
}

#[tauri::command]
async fn clear_events(state: tauri::State<'_, AsyncSender>) -> Result<(), ()> {
    let event_list = &mut state.vec.lock().await;

    event_list.clear();

    Ok(())
}

#[tauri::command]
async fn toggle_bot(toggle: i32, state: tauri::State<'_, AsyncSender>) -> Result<(), ()> {
    let main_thread_tx = state.inner.lock().await;

    main_thread_tx
        .send(toggle)
        .map_err(|e| {
            dbg!(e);
        })
        .unwrap();

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
            tauri::async_runtime::spawn(async move {
                loop {
                    match rx.try_recv() {
                        Ok(1) => {
                            dbg!(&event_list_rx.lock().await);
                            for e in event_list_rx.lock().await.iter() {
                                let mut rx_clone = tx.subscribe();

                                let key = e.key;
                                let intv = e.interval;

                                tauri::async_runtime::spawn(async move {
                                    let mut enigo = Enigo::new();
                                    let mut interval =
                                        time::interval(Duration::from_millis(intv as u64));

                                    while let Ok(_) = rx_clone.try_recv() {}

                                    loop {
                                        interval.tick().await;

                                        match rx_clone.try_recv() {
                                            Err(broadcast::error::TryRecvError::Empty) => {}
                                            _ => break,
                                        }

                                        enigo.key_click(Key::Layout(key));
                                    }
                                });
                            }
                        }
                        _ => continue,
                    }
                }
            });

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            create_event,
            toggle_bot,
            delete_event,
            clear_events
        ])
        .run(tauri::generate_context!())
        .expect("Tauri Application Error");
}
