# Endpoints
## User
### Create user
- `/create_user`
- Create user with given `username` `password` `nickname` and optionaly encoded `profile-picture`. See [[Images]]
- Example json:
```json
```

### Get user data
`/get_user_data`
returt user `nickname` `pfp` `username` based on JWT
Require logined user
Example request:
```http
GET /get_user_data HTTP/1.1
Host: url.com
authorization: <jwt_token>
```
response:
```json
{
	"nickname": "<nickname>",
	"nrofile Picture": "<profile_picture>",
	"username": "<username>",
}
```
### Login
**Post**
`/login`
Take credentials (`username` `password`) and return JWT token if allowed and error if not
Example request:
```http
POST http://url.com/login
{
	"username": "admin",
	"password": "admin1234"
}
```
Example response succes:
```http
HTTP/1.1 200 OK
{
	"Ok": "You succesfully logged in",
	"JWT": "<Your JWT token>,
}
```
Example response wrong credentials:
```http
HTTP/1.1 401 UNATHORIZED
{
	"Error": "Wrong credentials"
}
```
Example response failure:
```http
HTTP/1.1 500 INTERNAL_SERVER_ERROR
{
	"Error": "Something went wrong"
}
```

## Topics
### Create topic
**POST**
`/create_topic`
Create topic with given `name` and optionaly encoded image `image`. See [[Images]]
Require logined user
Example request:
```http
POST /create_topic HTTP/1.1
Host: url.com
authorization: <jwt_token>
Content-Type: application/json
{
    "name": "Space",
    "topic_picture": "<b64_encoded_image>"
}
```

### Get all topics
**GET**
`/get_topics`
Return all topics with `name` `no_spaces_name` and image name `topic_picture` which can be accesed on `/res/topic/<img_name>` see [[Images]]
Example response:
```json
[
	{
		"name":"Space",
		"no_spaces_name":"Space",
		"topic_picture":"cff86c44-f0be-4a19-83d2-95fe7dcb8c64"
	},
	{
		"name":"Microslop Haters",
		"no_spaces_name":"Microslop_Haters",
		"topic_picture":"41f07f81-27db-4698-9d02-38f3cdc502a1"
	}
]
```

## Chat
### Append message
**POST**
`/append_message`
Append message to given topic chat
Can contain optional image or message, will be ignored if image and messge will be empty.
Require logined user
```http
POST /create_topic HTTP/1.1
Host: url.com
authorization: <jwt_token>
Content-Type: application/json
{
    "topic": "Cars",
    "message": "<Your message>",
    "image": "<b64 encoded image>"
}
```

