import requests

url = "http://127.0.0.1:8000/create/category"

data = {
    "id": 0,
    "name": "category2",
    "description": "CATEGORY!"
}

r = requests.post(url=url, json=data)

print(r)