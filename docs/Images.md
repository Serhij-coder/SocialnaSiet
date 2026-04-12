## Image storage and acces
Images are saved to `DATA_DIR` defined in `.env`.
```env
DATA_DIR=./data
```
With **UUID** as a name
UUID also stored in db with its user or topic.
Client can get image from to `http://url.com/res/<image_uuid>`
**Example**: `http://url.com/res/asd345asd456`
![[Images 2026-04-11 14.21.19.excalidraw]]

## Transferring image
Client must encode image to **base64** and send as a json
base64 encoded image will be decoded on backend and stored.
#### Related [[endpoints|API endpoints]]
- Create user
- Create topic