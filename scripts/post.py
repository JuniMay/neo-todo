import requests

url = "http://127.0.0.1:8000/create/reminder-task"

data = {
    "id": 0,
    "title": "Reminder todo task",
    "description": "This is the first task",
    "deadline": "2023-05-13T19:20:30.45Z",
    "priority": 10,
    "status": "PENDING",
    "category_id": None,
    "remind_time": "2023-05-13T21:20:30.45Z",
}

r = requests.post(url=url, json=data)

print(r)