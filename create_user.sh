#!/bin/bash

username=$1
password=$2
name=$3

curl -X POST 127.0.0.1:3000/create_user \
  -H "Content-Type: application/json" \
  -d "{
      \"username\": \"$username\",
      \"password\": \"$password\",
      \"nickname\": \"$name\"
    }"
