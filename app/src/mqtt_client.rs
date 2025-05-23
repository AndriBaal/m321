use rumqttc::{MqttOptions, AsyncClient, QoS};
use crate::app::AppState;

pub async fn mqtt_client(app: AppState) {
	// let mut mqttoptions = MqttOptions::new(env!("CARGO_PKG_NAME"), &app.args.mosquitto_host, app.args.mosquitto_port);
	// mqttoptions.set_credentials(&app.args.mosquitto_user, &app.args.mosquitto_password);

	// let (client, mut eventloop) = AsyncClient::new(mqttoptions, 10);

	// client.subscribe("sensor/temperature", QoS::AtMostOnce).await.unwrap();

	// loop {
	// 	let event = eventloop.poll().await;
	// 	match event {
	// 		Ok(rumqttc::Event::Incoming(rumqttc::Packet::Publish(p))) => {
	// 			let json: serde_json::Value = serde_json::from_slice(&p.payload).unwrap();
	// 			println!("Received: {:?}", json);
	// 		}
	// 		_ => {}
	// 	}
	// }
}
