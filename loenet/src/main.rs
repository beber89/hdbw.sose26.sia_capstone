mod queue;
mod websocket;
mod local_interface;
mod config;
mod message;

use config::Config;
use std::sync::Arc;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let config = Config::default();
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

    // HTTP server (for local Python programs)
    local_interface::start_http_server(
        config.http_addr,
        outbound_queue,
        inbound_queue,
    ).await?;

    Ok(())
}
