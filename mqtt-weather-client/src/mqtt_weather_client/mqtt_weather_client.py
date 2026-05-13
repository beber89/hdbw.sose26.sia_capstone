import paho.mqtt.client as mqtt

BROKER = "broker.mqttdashboard.com"
PORT = 1883
TOPIC = "wokwi-weather"


def on_connect(client, userdata, flags, reason_code, properties):
    print(f"Connected with reason code: {reason_code}")
    client.subscribe(TOPIC)
    print(f"Subscribed to topic: {TOPIC}")


def on_message(client, userdata, message):
    payload = message.payload.decode("utf-8", errors="replace")
    print(f"[{message.topic}] {payload}")


client = mqtt.Client(mqtt.CallbackAPIVersion.VERSION2)

client.on_connect = on_connect
client.on_message = on_message

client.connect(BROKER, PORT, keepalive=60)
client.loop_forever()
