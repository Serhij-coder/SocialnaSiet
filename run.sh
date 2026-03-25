#!/bin/bash

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
  DATABASE_URL="postgresql://serhii@localhost:5432/postgres" sea-orm-cli migrate refresh

elif [[ $1 == "--entities" ]]; then
  echo "Generating entity files from the database schema..."
  sea-orm-cli generate entity -u postgresql://serhii@localhost:5432/postgres -o src/entities

elif [[ $1 == "--help" ]] || [[ $1 == "-h" ]]; then
  echo "$USAGE"

else
  echo "Invalid option: $1"
  echo USAGE
  exit 1
fi
