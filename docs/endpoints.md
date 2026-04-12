## Endpoints
### Create user
- `/create_user`
- Create user with given `username` `password` `nickname` and optionaly encoded `profile-picture`. See [[Images]]
- Example json:
```json
```

### Create topic
**POST**
`/create_topic`
Create topic with given `name` and optionaly encoded image `image`. See [[Images]]
Example request:
```json
{
	"name": "Space",
	"topic_picture": "<b64_encoded_image>"
}
```

### Get all topics
**GET**
`/get_topics`
Return all topics with `name` `no_spaces_name` and image name `topic_picture` which can be accesed on `/res/<img_name>` see [[Images]]
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