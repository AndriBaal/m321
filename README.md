# M321

## Documentation

...

## Video

Generell:
- Begründen warum sich für die einzelnen Technologien entschieden wurde
- Vor / Nachteile pro Technologie

## General
- Ziel
- Insgesamt 6 Services 

## Arduino
- Microcontroller
- DHT11 Temparatur sensor
- ESP 8266
- Wifi
- Schaltungzeigen

## Actix App
- Web App
- Rust
- Actix

## MongoDB
- Persistente Datenspeicherung
- Speicherung von Arduino Daten
- Speicherung von Sessions (Automatisches Löschen via Index)
- Auswerten von Durcchscnitt letzte 24 Stunden

## Mosquitto
- Mqtt Broker
- Passwortschützt
- Kommunikation zwischen Arduino und Web APP
- Datenübertragung über JSON (Sinnvoll weil MongoDB auch JSON verwendet)

## Nginx
- Middleman
- Eingebaut für zukünftige Erweiterungen wie:
    - Load Balancing
    - HTTPS
    - Sicherheit (wie z.B. DoS Attacken)
    - Cache

## Keycloak
- Authentication
- Benutzermanagement

## Systemarchitektur
- Grafik der Archtiektur

## Docker Setup
- docker compose
- secrets

## Ausblick
- Server Konfiguration mit https
- Service für Monitoring
- Anschliessen mehrerer Arduinos
