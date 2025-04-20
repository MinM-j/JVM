use crate::state::{Header, FILE_NAME, SERVER_STATE, VIS_BOOL};
use futures_util::{SinkExt, StreamExt};
use serde_json::json;
use tokio::fs::{self, OpenOptions};
use tokio::io::AsyncWriteExt;
use tokio::time::{self, Duration};
use tokio_tungstenite::{accept_async, tungstenite::protocol::Message};

pub async fn consumer_thread() {
    let vis_flag = VIS_BOOL.lock().unwrap();
    if *vis_flag {
        let addr = "127.0.0.1:8080"; // WebSocket server address
        let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
        println!("WebSocket server listening on {}", addr);

        if let Ok((stream, _)) = listener.accept().await {
            let ws_stream = accept_async(stream).await.unwrap();
            println!("New WebSocket connection established");

            let (mut write, read) = ws_stream.split();
            let mut interval = time::interval(Duration::from_secs(5));

            loop {
                interval.tick().await;
                let data_to_send = {
                    let mut queue = SERVER_STATE.lock().unwrap();
                    queue.pop_front()
                };

                match data_to_send {
                    Some(msg) => match msg.header {
                        Header::DATA => {
                            write
                                .send(Message::Text(msg.json.clone().into()))
                                .await
                                .unwrap();
                            println!("Sent data: {}", msg.json);
                        }
                        Header::EOF => {
                            write.send(Message::Close(None)).await.unwrap();
                            println!("EOF received, closing connection.");
                            break;
                        }
                    },
                    None => {
                        println!("Queue is empty, waiting for data...");
                    }
                }
            }
            println!("WebSocket connection closed.");
        }
    }
}

pub async fn file_writer() {
    let vis_flag = VIS_BOOL.lock().unwrap();
    if *vis_flag {
        let file_path = FILE_NAME.lock().unwrap();
        let mut file = OpenOptions::new()
            .write(true)
            .create(true)
            .truncate(true)
            .open(file_path.clone())
            .await
            .unwrap();
        file.write_all(b"[").await.unwrap();
        let mut first = true;
        loop {
            let data_to_write = {
                let mut queue = SERVER_STATE.lock().unwrap();
                queue.pop_front()
            };

            match data_to_write {
                Some(msg) => match msg.header {
                    Header::DATA => {
                        /*
                                            file.write_all(msg.json.as_bytes()).await.unwrap();
                                            file.write_all(b"\n").await.unwrap();
                        */
                        if !first {
                            file.write_all(b",\n").await.unwrap();
                        }
                        first = false;
                        file.write_all(msg.json.as_bytes()).await.unwrap();
                        //println!("Written data: {}", msg.json);
                    }
                    Header::EOF => {
                        //file.write_all(b"EOF\n").await.unwrap();
                        //println!("EOF received, writing EOF marker to file.");
                        break;
                    }
                },
                None => {
                    println!("Queue is empty, waiting for data...");
                }
            }
        }
        file.write_all(b"]").await.unwrap();
        println!("File writer finished.");
    }
}
