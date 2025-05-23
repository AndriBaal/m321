use rumqttc::{MqttOptions, AsyncClient, QoS};
use std::{sync::Arc, time::Duration};

use crate::app::AppState;

pub async fn mqtt_client(app: AppState) {
	let mut mqttoptions = MqttOptions::new("rust-client", "localhost", 1883);
	mqttoptions.set_keep_alive(Duration::from_secs(10));
	mqttoptions.set_credentials("mqtt-username", "mqtt-password");

	let (client, mut eventloop) = AsyncClient::new(mqttoptions, 10);

	client.subscribe("sensor/temperature", QoS::AtMostOnce).await.unwrap();

	loop {
		let event = eventloop.poll().await;
		match event {
			Ok(rumqttc::Event::Incoming(rumqttc::Packet::Publish(p))) => {
				println!("Received: {:?}", String::from_utf8_lossy(&p.payload));
			}
			_ => {}
		}
	}
}
