use bson::DateTime;
use rumqttc::{AsyncClient, MqttOptions, QoS};

use crate::{app::AppState, models::temperature_log::TemperatureLog};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
struct ArduinoData {
    client_name: String,
    temperature: i32,
    humidity: i32,
}

pub async fn mqtt_client(app: AppState) {
    let mut mqttoptions = MqttOptions::new(
        env!("CARGO_PKG_NAME"),
        &app.args.mosquitto_host,
        app.args.mosquitto_port,
    );
    mqttoptions.set_credentials(
        &app.args.mosquitto_user,
        std::fs::read_to_string(&app.args.mosquitto_password_file).unwrap(),
    );

    log::info!("Connecting MQTT");
    let (client, mut eventloop) = AsyncClient::new(mqttoptions, 10);

    client
        .subscribe("arduino/data", QoS::AtMostOnce)
        .await
        .unwrap();

    loop {
        let event = eventloop.poll().await;
        match event {
            Ok(rumqttc::Event::Incoming(rumqttc::Packet::Publish(p))) => {
                println!("Received Data: {}", String::from_utf8_lossy(&p.payload));
                let data: ArduinoData = serde_json::from_slice(&p.payload).unwrap();
                if data.humidity >= 0 && data.humidity <= 100 {
                    app.entries
                        .insert_one(TemperatureLog {
                            id: None,
                            client_name: data.client_name,
                            temperature: data.temperature,
                            humidity: data.humidity,
                            time: DateTime::now(),
                        })
                        .await
                        .unwrap();
                }
            }
            _ => {}
        }
    }
}
