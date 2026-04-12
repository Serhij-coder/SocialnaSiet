import requests
import sys
import base64


def create_user(username, password, nickname):
    url = "http://127.0.0.1:3000/create_user"

    image = encode_img()
    # image = ""
    # Python dictionaries map perfectly to JSON objects
    payload = {
        "username": username,
        "password": password,
        "nickname": nickname,
        "profile_picture": image,
    }

    try:
        # 'json=' automatically sets Content-Type to application/json
        response = requests.post(url, json=payload)

        # Check if the status code is 2xx
        if response.ok:
            print("Success:", response.json())
        else:
            print(f"Error {response.status_code}:", response.text)

    except requests.exceptions.RequestException as e:
        print(f"Connection failed: {e}")


def encode_img():
    with open("./johny.jpg", "rb") as image_file:
        # 1. Read the binary data
        binary_data = image_file.read()

        # 2. Encode to base64 bytes
        base64_bytes = base64.b64encode(binary_data)

        # 3. Decode to a UTF-8 string for JSON compatibility
        return base64_bytes.decode("utf-8")


if __name__ == "__main__":
    # Ensure all 3 arguments are provided (sys.argv[0] is the script name)
    if len(sys.argv) != 4:
        print("Usage: python script.py <username> <password> <nickname>")
    else:
        create_user(sys.argv[1], sys.argv[2], sys.argv[3])
