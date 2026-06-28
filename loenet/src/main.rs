mod queue;
mod websocket;
mod local_interface;
mod config;
mod message;

use config::Config;
use std::sync::Arc;
use std::env;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {

    let args: Vec<String> = env::args().collect();
    let port = args.get(1)
        .map(|s| s.parse::<u16>())          // Parse to Result<u16, _>
        .unwrap_or_else(|| Ok(8765))        // Fallback to Ok(8765) if no arg
        .unwrap_or_else(|_| {               // Handle parse error
            eprintln!("Invalid port. Using default (8765).");
            8765
        });
    let config = Config::new(port);

    let outbound_queue = Arc::new(queue::OutboundQueue::new());
    let inbound_queue = Arc::new(queue::InboundQueue::new());
    let outbound_queue_for_ws = Arc::clone(&outbound_queue);
    let inbound_queue_for_ws = Arc::clone(&inbound_queue);

    // WebSocket server (for inter-machine communication)
    tokio::spawn(async move {
        if let Err(e) = websocket::start_websocket_server(
            config.websocket_addr,
            outbound_queue_for_ws,
            inbound_queue_for_ws,
        ).await {
            eprintln!("WebSocket server error: {}", e);
        }
    });

    local_interface::start(
        outbound_queue,
        inbound_queue,
    ).await?;

    Ok(())
}
