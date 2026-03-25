#!/bin/bash

# Check if .env exists
if [ -f .env ]; then
  # 1. Read the file
  # 2. Filter out lines starting with # (comments)
  # 3. Export each line as an environment variable
  export $(grep -v '^#' .env | xargs)
  echo "Loaded environment from .env"
else
  echo ".env file not found"
fi

USAGE="
Usage: script [OPTIONS]

Options:
  --run        Start the application server
  --migrate    Execute pending database migrations
  --entities   Generate entity files from the database schema
"

if [[ $1 == "--run" ]]; then
  echo "Starting the application server..."
  cargo run

elif [[ $1 == "--migrate" ]]; then
  echo "Executing pending database migrations..."
  sea-orm-cli migrate refresh

elif [[ $1 == "--entities" ]]; then
  echo "Generating entity files from the database schema..."
  sea-orm-cli generate entity -u "$DATABASE_URL" -o src/entities

elif [[ $1 == "--viewdb" ]]; then
  rainfrog

elif [[ $1 == "--help" ]] || [[ $1 == "-h" ]]; then
  echo "$USAGE"

else
  echo "Invalid option: $1"
  echo "$USAGE"
  exit 1
fi
