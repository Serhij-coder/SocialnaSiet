## Stream
Stream response example:
- String can be empty it mean no content
- Image is **uuid** of image see [[Images]]
- Every 10 seconds stream will send content with string get from `/get_ignored_message`. This is control for listeners and should be ignored.
```json
{
	"message": "<message_string>",
	"image": "<Image_string>"
}
```
### More examples:
message only:
```json
{
	"image":null,
	"message":"dsffjgalkj;lkdsajf;lkjsajadsjf"
}
```