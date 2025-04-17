use crate::state::{Header, SERVER_STATE};
use futures_util::{SinkExt, StreamExt};
use tokio::time::{self, Duration};
use tokio_tungstenite::{accept_async, tungstenite::protocol::Message};

pub async fn consumer_thread() {
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
