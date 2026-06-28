use serde_json::Value;
use std::sync::Arc;
use tokio::net::TcpListener;
use crate::message::Message;
use tokio::net::UnixListener;


pub async fn start (
    outbound_queue: Arc<crate::queue::OutboundQueue>,
    inbound_queue: Arc<crate::queue::InboundQueue>,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    // TODO: New sock for incoming 
    // TODO: Generate random file each time
    let outgoing_listener = UnixListener::bind("/tmp/loenet_outbound.sock")?;
    loop {
        let (stream, _) = outgoing_listener.accept().await?;
        let outbound_queue_clone = Arc::clone(&outbound_queue);
        tokio::spawn(async move {
            // NOTE: MAXIMUM SIZE 
            let mut buf = [0u8; 1024];
            stream.readable().await.unwrap();
            let n = stream.try_read(&mut buf).unwrap();
            let json: Value = serde_json::from_slice(&buf[..n]).unwrap();
            println!("Received: {:?}", json);
            outbound_queue_clone.push(json);
        });
    }

    // let incoming_listener = UnixListener::bind("/tmp/loenet_inbound.sock")?;
    // loop {
    //     let (stream, _) = incoming_listener.accept().await?;
    //     let outbound_queue_clone = Arc::clone(&outbound_queue);
    //     tokio::spawn(async move {
    //         // NOTE: MAXIMUM SIZE 
    //         let mut buf = [0u8; 1024];
    //         stream.readable().await.unwrap();
    //         let n = stream.try_read(&mut buf).unwrap();
    //         let json: Value = serde_json::from_slice(&buf[..n]).unwrap();
    //         println!("Received: {:?}", json);
    //         outbound_queue_clone.push(json);
    //     });
    // }

    Ok(())
}

