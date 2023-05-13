import requests

url = "http://127.0.0.1:8000/create/common-task"

data = {
    "id": 3,
    "title": "Second todo task",
    "description": "This is the first task",
    "deadline": "2023-05-13T19:20:30.45Z",
    "priority": 10,
    "status": "PENDING",
    "category_id": None
}

requests.post(url=url, json=data)