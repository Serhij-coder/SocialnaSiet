POST http://localhost:3000/create_post
Authorization: {{jwt_token}}
Content-Type: application/json
{
    "topic": "General",
    "header": "Test Post Header",
    "content": "Test post content",
    "image": ""
}