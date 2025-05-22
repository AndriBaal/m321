#!/bin/bash

REAL_SECRET=$(cat /run/secrets/keycloak_secret)
REALM_SOURCE="/tmp/keycloak/m321-realm.json"
REALM_TARGET_FOLDER="/opt/keycloak/data/import"
REALM_TARGET="${REALM_TARGET_FOLDER}/m321-realm.json"

echo "Creating import directory if not exists..."
mkdir -p "$REALM_TARGET_FOLDER"

echo "Copying realm file to container-local path..."
cp "$REALM_SOURCE" "$REALM_TARGET"

echo "Injecting client secret..."
sed -i "s|__KEYCLOAK_CLIENT_SECRET__|$REAL_SECRET|g" "$REALM_TARGET"

echo "Starting Keycloak with start-dev and realm import..."
exec /opt/keycloak/bin/kc.sh start-dev --import-realm
