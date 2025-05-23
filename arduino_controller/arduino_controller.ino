#include <ESP8266WiFi.h>
#include <PubSubClient.h>
#include <DHT11.h>

#define DHTPIN 4
#define DHTTYPE DHT11

// WiFi credentials
const char* ssid = "crab";
const char* password = "crabbycrab";

// MQTT Broker
const char* mqtt_server = "MacBook-Pro-von-Andri";
const int mqtt_port = 1883;
const char* mqtt_user = "app";
const char* mqtt_pass = "mqtt-password";
const char* name = "arduino01";

WiFiClient wifiClient;
PubSubClient mqttClient(wifiClient);
DHT11 dht11(DHTPIN);

void connect_wifi() {
	WiFi.begin(ssid, password);
	while (WiFi.status() != WL_CONNECTED) {
		delay(500);
	}
}

void connect_mqtt() {
	while (!mqttClient.connected()) {
		if (mqttClient.connect(name, mqtt_user, mqtt_pass)) {
			// Connected
		} else {
			delay(500);
		}
	}
}

void setup() {
	connect_wifi();
	mqttClient.setServer(mqtt_server, mqtt_port);
}

void loop() {
	if (WiFi.status() != WL_CONNECTED) {
		connect_wifi();
	}
	if (!mqttClient.connected()) {
		connect_mqtt();
	}
	mqttClient.loop();

	int temp = dht11.readTemperature();
	int hum = dht11.readHumidity();
	char payload[128];
	snprintf(payload, sizeof(payload), "{\"client_name\":\"%s\",\"temperature\":%d,\"humidity\":%d}", name, temp, hum);
	mqttClient.publish("arduino/data", [temp, hum]);
	delay(5000);
}
