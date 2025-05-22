#!/bin/bash

REAL_SECRET=$(cat /run/secrets/keycloak_secret)
REALM_FILE="/opt/keycloak/data/import/m321-realm.json"

echo "Replacing client secret in realm file..."
sed -i "s|__KEYCLOAK_CLIENT_SECRET__|$REAL_SECRET|g" "$REALM_FILE"

echo "Starting Keycloak with start-dev and realm import..."
exec /opt/keycloak/bin/kc.sh start-dev --import-realm
