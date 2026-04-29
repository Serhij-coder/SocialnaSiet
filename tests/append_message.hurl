POST http://127.0.0.1:3000/append_message
Authorization: {{jwt_token}}
Content-Type: application/json
{
    "topic": "General",
    "message": "Lorem ipsum dolor sit amet, consectetur adipiscing elit. Donec a diam lectus. Sed sit amet ipsum mauris.",
    "image": ""
}
