#!/bin/bash

# Create secrets directory if it doesn't exist
mkdir -p ./secrets

# Array of secret filenames
secrets=("mongo_pass.txt" "session_secret.txt" "keycloak_secret.txt" "keycloak_admin_pass.txt" "keycloak_user_pass.txt")

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
mosquitto_user="app"

if [[ ! -f "$mosquitto_user_file" ]]; then
	mosquitto_pass=$(generate_password)

	# Generate a random 16-character salt from given character set
	chars='0123456789abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ'
	salt=$(LC_ALL=C tr -dc "$chars" </dev/urandom | head -c 16)

	# Base64-decode the salt (simulate same step as Python's base64.b64decode)
	# Note: This is not a typical use of base64 for salts, but matching the Python logic
	decoded_salt=$(echo -n "$salt" | base64 --decode 2>/dev/null || echo -n "")  # fallback to empty if decoding fails

	# Concatenate password and decoded salt
	input="$mosquitto_pass$decoded_salt"

	# Calculate SHA-512 hash in binary, then Base64 encode the result
	digest=$(echo -n "$input" | openssl dgst -sha512 -binary | base64)

	# Output similar to Python format: $6$salt$base64_encoded_digest
	echo -n "$mosquitto_pass" > "./secrets/mosquitto_pass.txt"
	echo "Created mosquitto_pass.txt"

	echo -n "${mosquitto_user}:\$6\$${salt}\$${digest}" > "./secrets/mosquitto_user.txt"
	echo "Created mosquitto_user.txt for user '$mosquitto_user'"

else
	echo "mosquitto_user.txt already exists. Skipping..."
fi


echo "Done"
