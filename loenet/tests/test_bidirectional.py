import requests
import json
import time
import threading
import subprocess
import os
import signal
import sys

def start_message_center():
    """Start the Rust Message Center in a subprocess."""
    return subprocess.Popen(
        ["cargo", "run"],
        cwd=os.path.join(os.path.dirname(__file__), ".."),
        preexec_fn=os.setsid
    )

def send_message():
    """Send a test message to the outbound queue."""
    message = {"test": "bidirectional", "data": {"value": 42}}
    response = requests.post("http://127.0.0.1:3000/send", json=message)
    print("Sent message:", response.status_code, message)

def receive_messages(stop_event):
    """Poll the inbound queue for messages."""
    while not stop_event.is_set():
        response = requests.post("http://127.0.0.1:3000/receive")
        if response.status_code == 200 and response.json():
            print("Received message:", response.json())
        time.sleep(0.5)

def test_bidirectional():
    """Test bidirectional communication."""
    # Start the Message Center
    process = start_message_center()
    time.sleep(2)  # Wait for server to start
    
    # Start receiver thread
    stop_event = threading.Event()
    receiver_thread = threading.Thread(target=receive_messages, args=(stop_event,))
    receiver_thread.start()
    
    try:
        # Send a message
        send_message()
        
        # Simulate receiving a message from another machine (for testing)
        # In a real scenario, this would come from the other machine via WebSocket
        test_message = {"test": "response", "data": {"value": 24}}
        requests.post("http://127.0.0.1:3000/send", json=test_message)  # Simulate inbound
        
        time.sleep(3)  # Wait for messages to propagate
        
    finally:
        # Cleanup
        stop_event.set()
        receiver_thread.join()
        os.killpg(os.getpgid(process.pid), signal.SIGTERM)
        process.wait()

if __name__ == "__main__":
    test_bidirectional()