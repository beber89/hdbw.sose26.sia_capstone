use tokio::net::{TcpListener, TcpStream};
use tokio_tungstenite::tungstenite::{protocol::Message, handshake::server::{Request, Response}};
use std::net::SocketAddr;
use std::sync::Arc;
use dashmap::DashMap;
use std::cell::RefCell;
use futures::{SinkExt, StreamExt};
use std::sync::Mutex;
use http::StatusCode;

thread_local! {
    static APP_NAME: RefCell<Option<String>> = RefCell::new(None);
}


lazy_static::lazy_static! {
    static ref APP_STREAMS: DashMap<String, Arc<Mutex<futures::stream::SplitSink<tokio_tungstenite::WebSocketStream<TcpStream>, Message>>>> = DashMap::new();
}

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

    let callback = |req: &Request, response: Response| {
        if let Some(app_data) = req.headers().get("X-App-Data") {
            match app_data.to_str() {
                Ok(app_name) => {
                    println!("Raw X-App-Data: {}", app_name);  
                    // Store `app_name` in thread-local storage as needed
                    APP_NAME.with(|cell| *cell.borrow_mut() = Some(app_name.to_string()));
                    return Ok(response);
                }
                Err(_) => {
                    let mut r = Response::builder()
                    .status(StatusCode::BAD_REQUEST)
                    .body(Some("Invalid UTF-8 in X-App-Data".to_string()))
                    .unwrap();
                    return Err(r);
                }, 
            }

        } else {
            let mut r = Response::builder()
            .status(StatusCode::BAD_REQUEST)
            .body(Some("Packet does not contain the applicaiton name in X-App-Data Header".to_string()))
            .unwrap();
            return Err(r);
        }
        Ok(response)
    };

    let ws_stream = tokio_tungstenite::accept_hdr_async(stream, callback)
        .await
        .expect("Error during WebSocket handshake");
    // Retrieve the appName and clear the thread-local storage
    let app_name = APP_NAME.with(|cell| cell.borrow_mut().take())
        .expect("appName not set in callback");

    // Store the ws_stream in the global map
    let (mut write, mut read) = ws_stream.split();
    APP_STREAMS.insert(app_name, Arc::new(Mutex::new(write)));


    // Forward outbound messages to the other machine
    println!("[handle_connection]:: before entering thread...");
    tokio::spawn(async move {
        eprintln!("[Thread] Forwarding outbound messages...");
        // TODO: Queue now should be specific for appName
        while let Some(message) = outbound_queue.pop() {
            let msg = Message::Text(message.to_string());
            eprintln!("[Thread] Sending message: {}", msg);
            // TODO:
            // if let Some(write) = APP_STREAMS.get("my_app") {
            //     println!("Found 'my_app': {:?}", write);
            //     if let Err(e) = write.send(msg).await {
            //         eprintln!("Failed to send message: {}", e);
            //         break;
            //     }
            // }


        }
    });

    // TODO: Read and put each msg into the queue of given app_name
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
    //TODO:
    // APP_STREAMS.remove(&app_name);
    Ok(())
}
