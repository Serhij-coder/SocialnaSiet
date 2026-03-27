import requests
import sys


def create_user(username, password, nickname):
    url = "http://127.0.0.1:3000/create_user"

    # Python dictionaries map perfectly to JSON objects
    payload = {"username": username, "password": password, "nickname": nickname}

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


if __name__ == "__main__":
    # Ensure all 3 arguments are provided (sys.argv[0] is the script name)
    if len(sys.argv) != 4:
        print("Usage: python script.py <username> <password> <nickname>")
    else:
        create_user(sys.argv[1], sys.argv[2], sys.argv[3])
