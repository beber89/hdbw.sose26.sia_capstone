import requests
import json
import time

# Poll the local Message Center for inbound messages
while True:
    response = requests.post("http://127.0.0.1:3000/receive")
    if response.status_code == 200 and response.json():
        print("Received from inbound queue:", response.json())
    time.sleep(1)