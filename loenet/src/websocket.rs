use futures::{SinkExt, StreamExt};
use tokio::net::{TcpListener, TcpStream};
use tokio_tungstenite::tungstenite::protocol::Message;
use std::net::SocketAddr;
use std::sync::Arc;

pub async fn start_websocket_server(
    addr: SocketAddr,
    outbound_queue: Arc<crate::queue::OutboundQueue>,
    inbound_queue: Arc<crate::queue::InboundQueue>,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let listener = TcpListener::bind(addr).await?;
    println!("WebSocket server listening on {}", addr);

    while let Ok((stream, _)) = listener.accept().await {
        println!("listener.accept()");
        let outbound_queue_clone = Arc::clone(&outbound_queue);
        let inbound_queue_clone = Arc::clone(&inbound_queue);
        tokio::spawn(async move {
            eprintln!("entered thread 2");
            if let Err(e) = handle_connection(stream, outbound_queue_clone, inbound_queue_clone).await {
                eprintln!("WebSocket connection error: {}", e);
            }
        });
    }
    Ok(())
}

async fn handle_connection(
    stream: TcpStream,
    outbound_queue: Arc<crate::queue::OutboundQueue>,
    inbound_queue: Arc<crate::queue::InboundQueue>,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let addr = stream.peer_addr().expect("Failed to get peer address");
    println!("New WebSocket connection: {}", addr);

    let ws_stream = tokio_tungstenite::accept_async(stream)
        .await
        .expect("Error during WebSocket handshake");

    let (mut write, mut read) = ws_stream.split();

    // Forward outbound messages to the other machine
    println!("[handle_connection]:: before entering thread...");
    tokio::spawn(async move {
        eprintln!("[Thread] Forwarding outbound messages...");
        while let Some(message) = outbound_queue.pop() {
            let msg = Message::Text(message.to_string());
            eprintln!("[Thread] Sending message: {}", msg);
            if let Err(e) = write.send(msg).await {
                eprintln!("Failed to send message: {}", e);
                break;
            }
        }
    });

    // Receive messages from the other machine and add to inbound queue
    while let Some(msg) = read.next().await {
        match msg {
            Ok(Message::Text(text)) => {
                match serde_json::from_str(&text) {
                    Ok(json) => {
                        inbound_queue.push(json);
                        println!("Added message to inbound queue");
                    }
                    Err(e) => eprintln!("Invalid JSON: {}", e),
                }
            }
            Ok(Message::Close(_)) => break,
            Err(e) => {
                eprintln!("WebSocket error: {}", e);
                break;
            }
            _ => {}
        }
    }
    Ok(())
}
