#[macro_use]
extern crate rocket;

use neo_todo::{CommonTask, DurationTask, ReminderTask, TaskID};
use rocket::{http::Status, response::Responder, serde::json::Json};
use rocket_db_pools::{
    sqlx::{self, Row},
    Connection, Database,
};

#[derive(Database)]
#[database("todo_db")]
struct Db(sqlx::MySqlPool);

#[derive(Debug, Clone, Copy)]
enum TodoError {
    NotFound,
    InternalServerError,
}

impl<'r> Responder<'r, 'static> for TodoError {
    fn respond_to(self, _: &'r rocket::Request<'_>) -> rocket::response::Result<'static> {
        match self {
            TodoError::NotFound => Err(Status::NotFound),
            TodoError::InternalServerError => Err(Status::InternalServerError),
        }
    }
}

#[get("/")]
async fn index(mut db: Connection<Db>) -> Result<Json<Vec<CommonTask>>, TodoError> {
    let result = sqlx::query(
        "
SELECT * FROM tbl_task",
    )
    .fetch_all(&mut *db)
    .await
    .map_err(|_| TodoError::InternalServerError)
    .map(|v| {
        let mut tasks = Vec::new();
        for row in v {
            tasks.push(CommonTask {
                id: row.get(0),
                title: row.get(1),
                description: row.get(2),
                deadline: row.get(3),
                priority: row.get(4),
                status: row.get(5),
                category_id: row.get(6),
            });
        }
        Json(tasks)
    });

    result
}

#[get("/all-common-tasks")]
async fn fetch_all_common_tasks(
    mut db: Connection<Db>,
) -> Result<Json<Vec<CommonTask>>, TodoError> {
    let result = sqlx::query(
        "
SELECT * FROM tbl_task",
    )
    .fetch_all(&mut *db)
    .await
    .map_err(|_| TodoError::InternalServerError)
    .map(|v| {
        let mut tasks = Vec::new();
        for row in v {
            tasks.push(CommonTask {
                id: row.get(0),
                title: row.get(1),
                description: row.get(2),
                deadline: row.get(3),
                priority: row.get(4),
                status: row.get(5),
                category_id: row.get(6),
            });
        }
        Json(tasks)
    });

    result
}

#[get("/common-task/<id>")]
async fn fetch_common_task(
    mut db: Connection<Db>,
    id: TaskID,
) -> Result<Json<CommonTask>, TodoError> {
    let result = sqlx::query(
        "
SELECT * FROM tbl_task WHERE task_id = ?",
    )
    .bind(id)
    .fetch_one(&mut *db)
    .await
    .map_err(|_| TodoError::NotFound)
    .map(|row| {
        Json(CommonTask {
            id: row.get(0),
            title: row.get(1),
            description: row.get(2),
            deadline: row.get(3),
            priority: row.get(4),
            status: row.get(5),
            category_id: row.get(6),
        })
    });
    result
}

#[get("/duration-task/<id>")]
async fn fetch_duration_task(
    mut db: Connection<Db>,
    id: TaskID,
) -> Result<Json<DurationTask>, TodoError> {
    let result = sqlx::query(
        "
SELECT 
    task_id, 
    task_title, 
    task_description, 
    task_deadline, 
    task_priority, 
    task_status, 
    category_id, 
    start_time, 
    end_time
FROM tbl_task NATURAL JOIN tbl_duration_task WHERE task_id = ?",
    )
    .bind(id)
    .fetch_one(&mut *db)
    .await
    .map_err(|_| TodoError::NotFound)
    .map(|row| {
        Json(DurationTask {
            id: row.get(0),
            title: row.get(1),
            description: row.get(2),
            deadline: row.get(3),
            priority: row.get(4),
            status: row.get(5),
            category_id: row.get(6),
            start_time: row.get(7),
            end_time: row.get(8),
        })
    });
    result
}

#[get("/reminder-task/<id>")]
async fn fetch_reminder_task(
    mut db: Connection<Db>,
    id: TaskID,
) -> Result<Json<ReminderTask>, TodoError> {
    let result = sqlx::query(
        "
SELECT 
    task_id, 
    task_title, 
    task_description, 
    task_deadline, 
    task_priority, 
    task_status, 
    category_id, 
    remind_time
FROM tbl_task NATURAL JOIN tbl_reminder_task WHERE task_id = ?",
    )
    .bind(id)
    .fetch_one(&mut *db)
    .await
    .map_err(|_| TodoError::NotFound)
    .map(|row| {
        Json(ReminderTask {
            id: row.get(0),
            title: row.get(1),
            description: row.get(2),
            deadline: row.get(3),
            priority: row.get(4),
            status: row.get(5),
            category_id: row.get(6),
            remind_time: row.get(7),
        })
    });
    result
}

#[get("/all-duration-tasks")]
async fn fetch_all_duration_tasks(
    mut db: Connection<Db>,
) -> Result<Json<Vec<DurationTask>>, TodoError> {
    let result = sqlx::query(
        "
SELECT 
    task_id, 
    task_title, 
    task_description, 
    task_deadline, 
    task_priority, 
    task_status, 
    category_id, 
    start_time, 
    end_time
FROM tbl_task NATURAL JOIN tbl_duration_task",
    )
    .fetch_all(&mut *db)
    .await
    .map_err(|_| TodoError::InternalServerError)
    .map(|v| {
        let mut tasks = Vec::new();
        for row in v {
            tasks.push(DurationTask {
                id: row.get(0),
                title: row.get(1),
                description: row.get(2),
                deadline: row.get(3),
                priority: row.get(4),
                status: row.get(5),
                category_id: row.get(6),
                start_time: row.get(7),
                end_time: row.get(8),
            });
        }
        Json(tasks)
    });
    result
}

#[get("/all-reminder-tasks")]
async fn fetch_all_reminder_tasks(
    mut db: Connection<Db>,
) -> Result<Json<Vec<ReminderTask>>, TodoError> {
    let result = sqlx::query(
        "
SELECT 
    task_id, 
    task_title, 
    task_description, 
    task_deadline, 
    task_priority, 
    task_status, 
    category_id, 
    remind_time
FROM tbl_task NATURAL JOIN tbl_reminder_task",
    )
    .fetch_all(&mut *db)
    .await
    .map_err(|_| TodoError::InternalServerError)
    .map(|v| {
        let mut tasks = Vec::new();
        for row in v {
            tasks.push(ReminderTask {
                id: row.get(0),
                title: row.get(1),
                description: row.get(2),
                deadline: row.get(3),
                priority: row.get(4),
                status: row.get(5),
                category_id: row.get(6),
                remind_time: row.get(7),
            });
        }
        Json(tasks)
    });
    result
}

#[post("/common-task", data = "<task>")]
async fn create_common_task(
    mut db: Connection<Db>,
    task: Json<CommonTask>,
) -> Result<(), TodoError> {
    let common_task = task.into_inner();
    let _id = common_task.id;
    let title = common_task.title.clone();
    let description = common_task.description.clone();
    let deadline = common_task.deadline;
    let priority = common_task.priority;
    let status = common_task.status;
    let category_id = common_task.category_id;

    let insert_result = sqlx::query(
        "
INSERT INTO tbl_task (
    task_title, 
    task_description, 
    task_deadline, 
    task_priority, 
    task_status, 
    category_id
) VALUES (?, ?, ?, ?, ?, ?)",
    )
    .bind(title)
    .bind(description)
    .bind(deadline)
    .bind(priority)
    .bind(status)
    .bind(category_id)
    .execute(&mut *db)
    .await
    .map_err(|_| TodoError::InternalServerError)?;

    let _last_id = insert_result.last_insert_id();

    Ok(())
}

#[post("/duration-task", data = "<task>")]
async fn create_duration_task(
    mut db: Connection<Db>,
    task: Json<DurationTask>,
) -> Result<(), TodoError> {
    let duration_task = task.into_inner();

    let _id = duration_task.id;
    let title = duration_task.title.clone();
    let description = duration_task.description.clone();
    let deadline = duration_task.deadline;
    let priority = duration_task.priority;
    let status = duration_task.status;
    let category_id = duration_task.category_id;
    let start_time = duration_task.start_time;
    let end_time = duration_task.end_time;

    let insert_result = sqlx::query(
        "
INSERT INTO tbl_task (
    task_title, 
    task_description, 
    task_deadline, 
    task_priority, 
    task_status, 
    category_id
) VALUES (?, ?, ?, ?, ?, ?)",
    )
    .bind(title)
    .bind(description)
    .bind(deadline)
    .bind(priority)
    .bind(status)
    .bind(category_id)
    .execute(&mut *db)
    .await
    .map_err(|_| TodoError::InternalServerError)?;

    let last_id = insert_result.last_insert_id();

    let _ = sqlx::query(
        "
INSERT INTO tbl_duration_task (
    task_id,
    start_time,
    end_time
) VALUES (?, ?, ?)",
    )
    .bind(last_id)
    .bind(start_time)
    .bind(end_time)
    .execute(&mut *db)
    .await
    .map_err(|_| TodoError::InternalServerError)?;

    Ok(())
}

#[post("/reminder-task", data = "<task>")]
async fn create_reminder_task(
    mut db: Connection<Db>,
    task: Json<ReminderTask>,
) -> Result<(), TodoError> {
    let reminder_task = task.into_inner();

    let _id = reminder_task.id;
    let title = reminder_task.title.clone();
    let description = reminder_task.description.clone();
    let deadline = reminder_task.deadline;
    let priority = reminder_task.priority;
    let status = reminder_task.status;
    let category_id = reminder_task.category_id;
    let remind_time = reminder_task.remind_time;

    let insert_result = sqlx::query(
        "
INSERT INTO tbl_task (
    task_title, 
    task_description, 
    task_deadline, 
    task_priority, 
    task_status, 
    category_id
) VALUES (?, ?, ?, ?, ?, ?)",
    )
    .bind(title)
    .bind(description)
    .bind(deadline)
    .bind(priority)
    .bind(status)
    .bind(category_id)
    .execute(&mut *db)
    .await
    .map_err(|_| TodoError::InternalServerError)?;

    let last_id = insert_result.last_insert_id();

    let _ = sqlx::query(
        "
INSERT INTO tbl_reminder_task (
    task_id,
    remind_time
) VALUES (?, ?)",
    )
    .bind(last_id)
    .bind(remind_time)
    .execute(&mut *db)
    .await
    .map_err(|_| TodoError::InternalServerError)?;

    Ok(())
}

#[rocket::main]
async fn main() -> anyhow::Result<()> {
    let rocket = rocket::build();
    rocket
        .attach(Db::init())
        .mount("/", routes![index,])
        .mount(
            "/create",
            routes![
                create_common_task,
                create_duration_task,
                create_reminder_task
            ],
        )
        .mount(
            "/fetch",
            routes![
                fetch_all_common_tasks,
                fetch_all_duration_tasks,
                fetch_all_reminder_tasks,
                fetch_common_task,
                fetch_duration_task,
                fetch_reminder_task,
            ],
        )
        .launch()
        .await?;
    Ok(())
}
