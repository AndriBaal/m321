#!/bin/bash

# Create secrets directory if it doesn't exist
mkdir -p ./secrets

# Array of secret filenames
secrets=("mongo_pass.txt" "session_secret.txt" "keycloak_secret.txt" "keycloak_admin.txt" "keycloak_test_user.txt")

# Function to generate a strong random password
generate_password() {
	LC_ALL=C tr -dc 'A-Za-z0-9' </dev/urandom | head -c 64
}

# Iterate over secrets and create files if they don't exist
for filename in "${secrets[@]}"; do
	filepath="./secrets/$filename"
	if [[ ! -f "$filepath" ]]; then
		password=$(generate_password)
		echo -n "$password" > "$filepath"
		echo "Created $filename"
	else
		echo "$filename already exists. Skipping..."
	fi
done

# Generate Mosquitto password hash using OpenSSL (SHA-512 crypt)
mosquitto_user_file="./secrets/mosquitto_user.txt"
mosquitto_user="admin"

if [[ ! -f "$mosquitto_user_file" ]]; then
	mosquitto_pass=$(generate_password)

	# Generate salt
	salt=$(openssl rand -base64 6 | tr -dc 'A-Za-z0-9' | head -c 16)

	# Hash password using SHA-512 crypt ($6$)
	hash=$(openssl passwd -6 -salt "$salt" "$mosquitto_pass")

	# Write to mosquitto_user.txt
	echo "$mosquitto_user:$hash" > "$mosquitto_user_file"
	echo "Created mosquitto_user.txt for user '$mosquitto_user'"
else
	echo "mosquitto_user.txt already exists. Skipping..."
fi


echo "Done"
