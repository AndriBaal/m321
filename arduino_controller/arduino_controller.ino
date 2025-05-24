#include <ESP8266WiFi.h>
#include <PubSubClient.h>
#include <DHT11.h>
#include "config.h"

WiFiClient wifiClient;
PubSubClient mqttClient(wifiClient);
DHT11 dht11(DHTPIN);

void connect_wifi() {
	WiFi.begin(WIFI_SSID, WIFI_PASSWORD);
	while (WiFi.status() != WL_CONNECTED) {
		delay(500);
	}
}

void connect_mqtt() {
	while (!mqttClient.connected()) {
		if (mqttClient.connect(DEVICE_NAME, MQTT_USER, MQTT_PASSWORD)) {
			// Connected
		} else {
			delay(500);
		}
	}
}

void setup() {
	Serial.begin(9600);
	connect_wifi();
	mqttClient.setServer(MQTT_SERVER, MQTT_PORT);
}

void loop() {
	if (WiFi.status() != WL_CONNECTED) {
		connect_wifi();
	}
	if (!mqttClient.connected()) {
		connect_mqtt();
	}
	mqttClient.loop();

	int temp;
	int hum;
	dht11.readTemperatureHumidity(temp, hum);
	
	char payload[128];
	snprintf(payload, sizeof(payload), "{\"client_name\":\"%s\",\"temperature\":%d,\"humidity\":%d}", DEVICE_NAME, temp, hum);
	Serial.println(payload);
	mqttClient.publish("arduino/data", payload);
	delay(SEND_RATE);
}
