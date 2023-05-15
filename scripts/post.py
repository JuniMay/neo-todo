import requests

base_url = "http://127.0.0.1:8000/"

create_url = f'{base_url}/create'
fetch_url = f'{base_url}/fetch'

duration_task_data = {
    'id': 0,
    'title': "#Duration-Task task with duration.",
    'description': "This is a task with duration.",
    'deadline': "2023-5-14T00:00:00Z",
    'status': "TODO",
    'category_id': None,
    'start_time': "2022-5-12T00:00:00Z",
    'end_time': "2022-5-14T00:00:00Z",
}

reminder_task_data = {
    'id': 0,
    'title': "#Reminder-Task task with remind.",
    'description': "This is a task with remind.",
    'deadline': "2023-5-14T00:00:00Z",
    'status': "REMINDED",
    'category_id': None,
    'remind_time': "2022-5-12T00:00:00Z",
}

common_task_data = {
    'id': 0,
    'title': "#Common-Task A common task.",
    'description': "This is a common task.",
    'deadline': "2023-5-14T23:11:35Z",
    'status': "DOING",
    'category_id': None,
    
    'kind': 0,
}

tag_data = {
    'id': 0,
    'name': "A tag",
}

category_data = {
    'id': 0,
    'name': "A category",
    'description': "Category is used to sort the tasks."
}

r = requests.post(url=f'{create_url}/duration-task', json=duration_task_data)
print(r)

r = requests.post(url=f'{create_url}/reminder-task', json=reminder_task_data)
print(r)

r = requests.post(url=f'{create_url}/common-task', json=common_task_data)
print(r)

r = requests.post(url=f'{create_url}/tag', json=tag_data)
print(r)

r = requests.post(url=f'{create_url}/category', json=category_data)
print(r)

duration_task_data = {
    'id': 0,
    'title': "#Duration-Task task with duration.",
    'description': "This is a task with duration with tag.",
    'deadline': "2023-5-14T00:00:00Z",
    'status': "TODO",
    'category_id': 1,
    'start_time': "2022-5-12T00:00:00Z",
    'end_time': "2022-5-14T00:00:00Z",
}

reminder_task_data = {
    'id': 0,
    'title': "#Reminder-Task task with remind.",
    'description': "This is a task with remind with tag.",
    'deadline': "2023-5-14T00:00:00Z",
    'status': "REMINDED",
    'category_id': 1,
    'remind_time': "2022-5-12T00:00:00Z",
}

common_task_data = {
    'id': 0,
    'title': "#Common-Task A common task.",
    'description': "This is a common task with tag!.",
    'deadline': "2023-5-14T23:11:35Z",
    'status': "DOING",
    'category_id': 1,
    'kind': 0,
}

r = requests.post(url=f'{create_url}/duration-task', json=duration_task_data)
print(r)

r = requests.post(url=f'{create_url}/reminder-task', json=reminder_task_data)
print(r)

r = requests.post(url=f'{create_url}/common-task', json=common_task_data)
print(r)
