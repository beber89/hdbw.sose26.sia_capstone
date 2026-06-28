import asyncio
import websockets

async def connect_to_rust_server():
    uri = "ws://localhost:8765"  # Replace with the Rust server's IP if remote
    try:
        async with websockets.connect(uri) as websocket:
            print("Connected to Rust WebSocket server!")
            # Keep the connection open (or add logic to send/receive messages)
            await asyncio.Future()  # Run forever
    except Exception as e:
        print(f"Connection failed: {e}")

if __name__ == "__main__":
    asyncio.run(connect_to_rust_server())
