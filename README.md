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

To operate the Arduino, you need to adjust the configuration in the `arduino_controller/config.h` file. Set the following parameters:
- WIFI_SSID
- WIFI_PASSWORD
- MQTT_SERVER
- MQTT_PASSWORD
