#include <ESP8266WiFi.h>
#include <PubSubClient.h>
#include <DHT11.h>

#define DHTPIN 4
#define DHTTYPE DHT11
DHT11 dht11(DHTPIN);

// WiFi credentials
const char* ssid = "your-ssid";
const char* password = "your-wifi-password";

// MQTT Broker
const char* mqtt_server = "broker-ip-or-hostname";
const int mqtt_port = 1883;
const char* mqtt_user = "mqtt-username";
const char* mqtt_pass = "mqtt-password";

WiFiClient espClient;
PubSubClient client(espClient);

void setup_wifi() {
	WiFi.begin(ssid, password);
	while (WiFi.status() != WL_CONNECTED) {
		delay(500);
	}
}

void reconnect() {
	while (!client.connected()) {
		if (client.connect("arduinoClient", mqtt_user, mqtt_pass)) {
			// Connected
		} else {
			delay(5000);
		}
	}
}

void setup() {
	Serial.begin(9600);
	setup_wifi();
	client.setServer(mqtt_server, mqtt_port);
}

void loop() {
	if (!client.connected()) {
		reconnect();
	}
	client.loop();

	float temp = dht11.readTemperature();
	if (!isnan(temp)) {
		char tempStr[8];
		dtostrf(temp, 1, 2, tempStr);
		client.publish("sensor/temperature", tempStr);
	}
	delay(5000);
}
