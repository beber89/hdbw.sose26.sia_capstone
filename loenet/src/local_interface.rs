use axum::{
    routing::post,
    Router,
    Json,
    extract::State,
};
use serde_json::Value;
use std::net::SocketAddr;
use std::sync::Arc;
use tokio::net::TcpListener;
use crate::message::Message;

pub async fn start_http_server(
    addr: SocketAddr,
    outbound_queue: Arc<crate::queue::OutboundQueue>,
    inbound_queue: Arc<crate::queue::InboundQueue>,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let app = Router::new()
        .route("/send", post(handle_send))
        .route("/receive", post(handle_receive))
        .with_state((outbound_queue, inbound_queue));

    println!("HTTP server listening on {}", addr);
    let listener = TcpListener::bind(addr).await?;
    axum::serve(listener, app).await?;
    Ok(())
}

async fn handle_send(
    State((outbound_queue, _)): State<(Arc<crate::queue::OutboundQueue>, Arc<crate::queue::InboundQueue>)>,
    Json(payload): Json<Message>,
) {
    outbound_queue.push(payload);
    println!("Added message to outbound queue");
}

async fn handle_receive(
    State((_, inbound_queue)): State<(Arc<crate::queue::OutboundQueue>, Arc<crate::queue::InboundQueue>)>,
) -> Json<Option<Value>> {
    Json(inbound_queue.pop())
}
