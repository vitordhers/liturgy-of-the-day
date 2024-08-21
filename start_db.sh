#!/bin/bash

# Load environment variables from .env file
export $(grep -v '^#' .env | xargs)

# Use the environment variable in your command
echo "Running command with SURREAL_DB_USER=$SURREAL_DB_USER"
surreal start --log debug --user $SURREAL_DB_USER --pass $SURREAL_DB_PASSWORD --auth --bind 0.0.0.0:8080 file://db/data