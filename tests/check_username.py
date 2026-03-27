import requests
import sys
import json


def check_username(username):
    url = "http://127.0.0.1:3000/check_username_availability"
    payload = {"username": username}

    try:
        response = requests.post(url, json=payload)

        # --- FULL RESPONSE PRINT ---
        print("-" * 30)
        print(f"DEBUG: Full Response from {url}")
        print(f"Status Code: {response.status_code}")
        print(f"Headers: {dict(response.headers)}")
        try:
            # Try to print pretty-formatted JSON if possible
            print(f"Body: {json.dumps(response.json(), indent=2)}")
        except:
            # Fallback to raw text if it's not JSON
            print(f"Body (Raw): {response.text}")
        print("-" * 30)
        # ---------------------------

        if response.status_code == 200:
            print(f"✅ Success: Available")
        elif response.status_code == 422:
            print(f"❌ Taken: {response.json().get('Error', 'Username unavailable')}")
        else:
            print(f"⚠️ Unexpected Status: {response.status_code}")

    except requests.exceptions.ConnectionError:
        print("🚫 Could not connect to the Rust server. Is it running?")


if __name__ == "__main__":
    if len(sys.argv) < 2:
        print("Usage: python check_user.py <username>")
    else:
        check_username(sys.argv[1])
