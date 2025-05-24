# M321

## How to Run

### Services

To run the services, several secrets must be defined in the `secrets` folder. You can automatically generate them by executing the `generate_secrets.sh` script:

```bash
chmod +x ./generate_secrets.sh
./generate_secrets.sh
```

Once the secrets are in place, start the containers with Docker Compose:

```bash
docker compose up --build
```

### Arduino

To operate the Arduino, update the configuration in the `arduino_controller/config.h` file by setting the following parameters:

- `WIFI_SSID`
- `WIFI_PASSWORD`
- `MQTT_SERVER` (your server's IP address)
- `MQTT_PASSWORD` (located in `./secrets/mosquitto_pass.txt`)

After configuring, upload `arduino_controller/arduino_controller.ino` to your `ESP8266` board using the provided `libraries` and a connected DHT11 sensor.
