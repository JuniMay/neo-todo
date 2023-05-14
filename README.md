# Neo TODO

Yet another todo application.

## Backend

Available requests are listed as below:

- `/`: List all tasks as common task.
  - `create/`
    - `common-task`: POST, create a task by the given json.
    - `duration-task`: POST, create a duration task by the given json.
    - `reminder-task`: POST, create a reminder task by the given json.
    - `tag`: POST, create a tag by the given json.
    - `category`: POST create a category by the given json.
  - `fetch/`
    - `all-common-tasks`: GET, get all tasks in common format.
    - `all-duration-tasks`: GET, get all duration tasks.
    - `all-reminder-tasks`: GET, get all reminder tasks.
    - `common-task?<id>`: GET, get the task by id.
    - `duration-task?<id>`: GET, get the duration task by id.
    - `reminder-task?<id>`: GET, get the reminder task by id.
  - `delete/`
    - `tag?<id>`: GET, delete the tag.
    - `category?<id>`: GET, delete the category.
    - `reminder-task?<id>`: GET, delete the reminder-task.
    - `duration-task?<id>`: GET, delete the duration-task.
    - `common-task?<id>`: GET, delete the common-task.
    - `task-tag?<task_id>&<tag_id>`: GET, delete the relation that task is of tag by the ID.
  - `update/`
    - `to-common-task`: POST, convert to common task by the given json.
    - `to-duration-task`: POST, convert to duration task by the given json.
    - `to-reminder-task`: POST, convert to reminder task by the given json.
    - `update-duration-task`: POST, update the columns in the duration task table.
    - `update-reminder-task`: POST, update the columns in the reminder task table.
    - `update-common-task`: POST, update the columns in the common task table.
    - `add-task-tag`: POST, add task-tag relation by the given json with task and tag id.
