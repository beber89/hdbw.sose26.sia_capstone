import asyncio
import websockets

async def receive_messages(websocket):
    """Background task to print incoming messages."""
    while True:
        try:
            async for message in websocket:
                print(f"Received: {message}")
        except websockets.exceptions.ConnectionClosed:
            print("Connection closed by server.")
        finally:
            print("Listening...")
            sleep(1)

async def connect_to_rust_server():
    uri = "ws://localhost:8765"  # Replace with the Rust server's IP if remote
    try:
        async with websockets.connect(uri) as websocket:
            print("Connected to Rust WebSocket server!")
            # Start the message receiver task
            receiver_task = asyncio.create_task(receive_messages(websocket))
            # Keep the connection open
            await asyncio.Future()  # Run forever
    except Exception as e:
        print(f"Connection failed: {e}")

if __name__ == "__main__":
    asyncio.run(connect_to_rust_server())
