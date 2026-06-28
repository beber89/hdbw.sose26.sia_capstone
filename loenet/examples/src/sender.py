import json
import time
import socket


# Send a message to the local Message Center (outbound)
sock = socket.socket(socket.AF_UNIX, socket.SOCK_STREAM)
sock.connect("/tmp/loenet_outbound.sock")
message = {"source": "program1", "data": {"key": "value"}}
sock.sendall(json.dumps(message).encode())

print("Sent to outbound queue: ")
