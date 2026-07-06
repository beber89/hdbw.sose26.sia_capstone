import paho.mqtt.client as mqtt

def on_connect(client, userdata, flags, rc):
    print(f"Connected with result code {rc}")
    client.subscribe("place/roof")

def on_message(client, userdata, msg):
    print(f"Topic: {msg.topic}, Message: {msg.payload.decode()}")

def main() -> int:
    client = mqtt.Client()
    client.on_connect = on_connect
    client.on_message = on_message

    client.connect("212.147.230.126", 1883, 60)
    client.loop_forever()
    return 0
