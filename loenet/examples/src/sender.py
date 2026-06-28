import requests
import json
import time

# Send a message to the local Message Center (outbound)
message = {"source": "program1", "data": {"key": "value"}}
response = requests.post("http://127.0.0.1:3000/send", json=message)
print("Sent to outbound queue:", response.status_code)
