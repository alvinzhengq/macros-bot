// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::{sync::Arc, time::Duration};
use tokio::{
    sync::broadcast,
    sync::mpsc::{self, error::TryRecvError},
    sync::Mutex,
    time,
};

struct AsyncSender {
    inner: Mutex<mpsc::Sender<Event>>,
}

struct Event {
    key: String,
    interval: i32,
}

#[tauri::command]
async fn create_event(
    key: &str,
    interval: i32,
    state: tauri::State<'_, AsyncSender>,
) -> Result<(), String> {
    let main_thread_tx = state.inner.lock().await;

    main_thread_tx
        .send(Event {
            key: String::from(key),
            interval,
        })
        .await
        .map_err(|e| e.to_string())
}

fn main() {
    let (tx, mut rx) = mpsc::channel(64);
    let (tx2, _) = broadcast::channel(64);

    tauri::Builder::default()
        .manage(AsyncSender {
            inner: Mutex::new(tx),
        })
        .setup(|_| {
            let event_list: Arc<Mutex<Vec<Event>>> = Arc::new(Mutex::new(vec![]));
            let event_list_main = event_list.clone();
            let mut toggled = false;

            tauri::async_runtime::spawn(async move {
                loop {
                    match rx.try_recv() {
                        Ok(e) => event_list_main.lock().await.push(e),
                        Err(TryRecvError::Empty) => {}
                        Err(TryRecvError::Disconnected) => break,
                    };

                    // hot key logic here
                    if !toggled {
                        let mut rx2_clone = tx2.subscribe();
                        let event_list_child = event_list.clone();

                        tauri::async_runtime::spawn(async move {
                            let mut interval = time::interval(Duration::from_millis(100));
                            let mut count = 1;

                            loop {
                                interval.tick().await;
                                match rx2_clone.try_recv() {
                                    Err(broadcast::error::TryRecvError::Empty) => {}
                                    _ => break,
                                }

                                for e in event_list_child.lock().await.iter() {
                                    if count % e.interval == 0 {
                                        // key event logic here
                                    }
                                }

                                count += 1;
                            }
                        });

                        toggled = true;
                    } else {
                        tx2.send(0).unwrap();
                    }
                }
            });

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![create_event])
        .run(tauri::generate_context!())
        .expect("Tauri Application Error");
}
