use paho_mqtt as mqtt;
use std::process;

fn main() {
    // Create a client & define connect options
    let cli = mqtt::Client::new("tcp://0.0.0.0:1883").unwrap_or_else(|err| {
        eprintln!("Error creating client: {}", err);
        process::exit(1);
    });

    let conn_opts = mqtt::ConnectOptionsBuilder::new()
        .keep_alive_interval(std::time::Duration::from_secs(20))
        .clean_session(true)
        .finalize();

    // Connect and subscribe
    if let Err(e) = cli.connect(conn_opts) {
        eprintln!("Error connecting to broker: {}", e);
        process::exit(1);
    }

    cli.subscribe("topic/", 1).unwrap_or_else(|err| {
        eprintln!("Error subscribing to topic: {}", err);
        process::exit(1);
    });

    // Define message callback
    cli.set_message_callback(|_, msg| {
        if let Some(msg) = msg {
            println!("Received: {}", msg);
        }
    });

    // Wait for messages
    loop {
        std::thread::sleep(std::time::Duration::from_millis(100));
    }
}
