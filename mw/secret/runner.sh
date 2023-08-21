#!/bin/bash

# Provide key to decrypt the env file.
if [ -z "$1" ]
  then
    echo "Please parse the private key to decrypt environment...👺"
    exit
fi

echo "Decrypting the file...🤠"
sleep 1

# Decrypt and move to .env.
openssl rsautl -decrypt -inkey $1 -in secret/env.enc > .dev.container/.config/.env

# Check if that was successful.
if [ $? -eq 0 ] 
then 
  echo "Successfully created environment...😼"
  sleep 1
else 
  echo "Could not create environment file. Decryption failed...💀" >&2
  exit
fi

# Health check docker on machine.
docker version > /dev/null

# Check if that was successful.
if [ $? -eq 0 ] 
then 
  echo "Docker daemon found...😼" 
  sleep 1
else 
  echo "Docker not found on the system...💀" >&2
  exit
fi

# Now run the docker compose and wait.
docker compose -f .dev.container/docker-compose.yaml up

