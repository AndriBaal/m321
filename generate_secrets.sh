#!/bin/bash

# Create secrets directory if it doesn't exist
mkdir -p ./secrets

# Array of secret filenames
secrets=("mongo_pass.txt" "session_secret.txt" "keycloak_secret.txt")

# Function to generate a strong random password
generate_password() {
	LC_ALL=C tr -dc 'A-Za-z0-9@#$%&*_+=' </dev/urandom | head -c 32
}

# Iterate over secrets and create files if they don't exist
for filename in "${secrets[@]}"; do
	filepath="./secrets/$filename"
	if [[ ! -f "$filepath" ]]; then
		password=$(generate_password)
		echo "$password" > "$filepath"
		echo "Created $filename"
	else
		echo "$filename already exists. Skipping..."
	fi
done

echo "Done"
