#[macro_use]
extern crate rocket;

use chrono::prelude::*;
use neo_todo::{
    Category, CategoryID, CommonTask, DurationTask, ReminderTask, Tag, TagID, TaskID, TaskTag,
};
use rocket::{http::Status, response::Responder, serde::json::Json};
use rocket_db_pools::{
    sqlx::{self, Row},
    Connection, Database,
};
use serde::{Deserialize, Serialize};
use sqlx::Acquire;

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

impl From<sqlx::Error> for TodoError {
    fn from(_: sqlx::Error) -> Self {
        TodoError::InternalServerError
    }
}

#[get("/")]
async fn index(mut db: Connection<Db>) -> Result<Json<Vec<CommonTask>>, TodoError> {
    let result = sqlx::query(
        "
SELECT * FROM tbl_task",
    )
    .fetch_all(&mut *db)
    .await?
    .into_iter()
    .map(|row| CommonTask {
        id: row.get(0),
        title: row.get(1),
        description: row.get(2),
        deadline: row.get(3),
        priority: row.get(4),
        status: row.get(5),
        category_id: row.get(6),
    })
    .collect();

    Ok(Json(result))
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
    .await?
    .into_iter()
    .map(|row| CommonTask {
        id: row.get(0),
        title: row.get(1),
        description: row.get(2),
        deadline: row.get(3),
        priority: row.get(4),
        status: row.get(5),
        category_id: row.get(6),
    })
    .collect();

    Ok(Json(result))
}

#[get("/common-task?<id>")]
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

#[get("/duration-task?<id>")]
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
FROM v_duration_task WHERE task_id = ?",
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

#[get("/reminder-task?<id>")]
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
FROM v_reminder_task WHERE task_id = ?",
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
FROM v_duration_task",
    )
    .fetch_all(&mut *db)
    .await?
    .into_iter()
    .map(|row| DurationTask {
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
    .collect();

    Ok(Json(result))
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
FROM v_reminder_task",
    )
    .fetch_all(&mut *db)
    .await?
    .into_iter()
    .map(|row| ReminderTask {
        id: row.get(0),
        title: row.get(1),
        description: row.get(2),
        deadline: row.get(3),
        priority: row.get(4),
        status: row.get(5),
        category_id: row.get(6),
        remind_time: row.get(7),
    })
    .collect();

    Ok(Json(result))
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
    .await?;

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
    .await?;

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
    .await?;

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
    .await?;

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
    .await?;

    Ok(())
}

#[post("/tag", data = "<tag>")]
async fn create_tag(mut db: Connection<Db>, tag: Json<Tag>) -> Result<(), TodoError> {
    let tag = tag.into_inner();
    let _tag_id = tag.id;
    let tag_name = tag.name;

    let _insert_result = sqlx::query(
        "
INSERT INTO tbl_tag (
    tag_name
) VALUES (?)",
    )
    .bind(tag_name)
    .execute(&mut *db)
    .await?;

    Ok(())
}

#[post("/category", data = "<category>")]
async fn create_category(
    mut db: Connection<Db>,
    category: Json<Category>,
) -> Result<(), TodoError> {
    let category = category.into_inner();
    let _category_id = category.id;
    let category_name = category.name;
    let category_description = category.description;

    let _insert_result = sqlx::query(
        "
INSERT INTO tbl_category (
    category_name,
    category_description
) VALUES (?, ?)",
    )
    .bind(category_name)
    .bind(category_description)
    .execute(&mut *db)
    .await?;

    Ok(())
}

#[get("/tag?<id>")]
async fn delete_tag(mut db: Connection<Db>, id: TagID) -> Result<(), TodoError> {
    let mut tx = db.begin().await?;

    let _delete_task_tag_result = sqlx::query(
        "
DELETE FROM tbl_task_tag WHERE tag_id = ?",
    )
    .bind(id)
    .execute(&mut tx)
    .await?;

    let _delete_tag_result = sqlx::query(
        "
DELETE FROM tbl_tag WHERE tag_id = ?",
    )
    .bind(id)
    .execute(&mut tx)
    .await?;

    tx.commit().await?;

    Ok(())
}

#[get("/category?<id>")]
async fn delete_category(mut db: Connection<Db>, id: CategoryID) -> Result<(), TodoError> {
    let mut tx = db.begin().await?;

    let _set_category_null_result = sqlx::query(
        "
UPDATE tbl_task SET category_id = NULL WHERE category_id = ?",
    )
    .bind(id)
    .execute(&mut tx)
    .await?;

    let _delete_category_result = sqlx::query(
        "
DELETE FROM tbl_category WHERE category_id = ?",
    )
    .bind(id)
    .execute(&mut tx)
    .await?;

    tx.commit().await?;

    Ok(())
}

#[get("/reminder-task?<id>")]
async fn delete_reminder_task(mut db: Connection<Db>, id: TaskID) -> Result<(), TodoError> {
    let mut tx = db.begin().await?;

    let _delete_reminder_result = sqlx::query(
        "
DELETE FROM tbl_reminder_task WHERE task_id = ?",
    )
    .bind(id)
    .execute(&mut tx)
    .await?;

    let _delete_task_tag_result = sqlx::query(
        "
DELETE FROM tbl_task_tag WHERE task_id = ?",
    )
    .bind(id)
    .execute(&mut tx)
    .await?;

    let _delete_task_result = sqlx::query(
        "
DELETE FROM tbl_task WHERE task_id = ?",
    )
    .bind(id)
    .execute(&mut tx)
    .await?;

    tx.commit().await?;

    Ok(())
}

#[get("/duration-task?<id>")]
async fn delete_duration_task(mut db: Connection<Db>, id: TaskID) -> Result<(), TodoError> {
    let mut tx = db.begin().await?;

    let _delete_duration_result = sqlx::query(
        "
DELETE FROM tbl_duration_task WHERE task_id = ?",
    )
    .bind(id)
    .execute(&mut tx)
    .await?;

    let _delete_task_tag_result = sqlx::query(
        "
DELETE FROM tbl_task_tag WHERE task_id = ?",
    )
    .bind(id)
    .execute(&mut tx)
    .await?;

    let _delete_task_result = sqlx::query(
        "
DELETE FROM tbl_task WHERE task_id = ?",
    )
    .bind(id)
    .execute(&mut tx)
    .await?;

    tx.commit().await?;

    Ok(())
}

#[get("/common-task?<id>")]
async fn delete_common_task(mut db: Connection<Db>, id: TaskID) -> Result<(), TodoError> {
    let mut tx = db.begin().await?;

    let _delete_task_tag_result = sqlx::query(
        "
DELETE FROM tbl_task_tag WHERE task_id = ?",
    )
    .bind(id)
    .execute(&mut tx)
    .await?;

    let _delete_task_result = sqlx::query(
        "
DELETE FROM tbl_task WHERE task_id = ?",
    )
    .bind(id)
    .execute(&mut tx)
    .await?;

    tx.commit().await?;

    Ok(())
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct ConversionToCommonTask {
    id: TaskID,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct UpdateDurationTask {
    id: TaskID,
    start_time: DateTime<Utc>,
    end_time: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct UpdateReminderTask {
    id: TaskID,
    remind_time: DateTime<Utc>,
}

#[post("/to-common-task", data = "<conversion>")]
async fn convert_to_common_task(
    mut db: Connection<Db>,
    conversion: Json<ConversionToCommonTask>,
) -> Result<(), TodoError> {
    let _result = sqlx::query(
        "
CALL to_common_task(?)",
    )
    .bind(conversion.id)
    .execute(&mut *db)
    .await?;

    Ok(())
}

#[post("/to-duration-task", data = "<conversion>")]
async fn convert_to_duration_task(
    mut db: Connection<Db>,
    conversion: Json<UpdateDurationTask>,
) -> Result<(), TodoError> {
    let _result = sqlx::query(
        "
CALL to_duration(?, ?, ?)",
    )
    .bind(conversion.id)
    .bind(conversion.start_time)
    .bind(conversion.end_time)
    .execute(&mut *db)
    .await?;

    Ok(())
}

#[post("/to-reminder-task", data = "<conversion>")]
async fn convert_to_reminder_task(
    mut db: Connection<Db>,
    conversion: Json<UpdateReminderTask>,
) -> Result<(), TodoError> {
    let _result = sqlx::query(
        "
CALL to_reminder(?, ?)",
    )
    .bind(conversion.id)
    .bind(conversion.remind_time)
    .execute(&mut *db)
    .await?;

    Ok(())
}

#[post("/update-reminder-task", data = "<update>")]
async fn update_reminder_task(
    mut db: Connection<Db>,
    update: Json<UpdateReminderTask>,
) -> Result<(), TodoError> {
    let _result = sqlx::query(
        "
CALL update_reminder(?, ?)",
    )
    .bind(update.id)
    .bind(update.remind_time)
    .execute(&mut *db)
    .await?;

    Ok(())
}

#[post("/update-duration-task", data = "<update>")]
async fn update_duration_task(
    mut db: Connection<Db>,
    update: Json<UpdateDurationTask>,
) -> Result<(), TodoError> {
    let _result = sqlx::query(
        "
CALL update_duration(?, ?, ?)",
    )
    .bind(update.id)
    .bind(update.start_time)
    .bind(update.end_time)
    .execute(&mut *db)
    .await?;

    Ok(())
}

#[post("/update-common-task", data = "<update>")]
async fn update_common_task(
    mut db: Connection<Db>,
    update: Json<CommonTask>,
) -> Result<(), TodoError> {
    let _result = sqlx::query(
        "
CALL update_common(?, ?, ?, ?, ?, ?, ?)",
    )
    .bind(update.id)
    .bind(update.title.clone())
    .bind(update.description.clone())
    .bind(update.deadline)
    .bind(update.priority)
    .bind(update.status.clone())
    .bind(update.category_id)
    .execute(&mut *db)
    .await?;

    Ok(())
}

#[post("/add-task-tag", data = "<addition>")]
async fn add_task_tag(mut db: Connection<Db>, addition: Json<TaskTag>) -> Result<(), TodoError> {
    let _result = sqlx::query(
        "
CALL add_task_tag(?, ?)",
    )
    .bind(addition.task_id)
    .bind(addition.tag_id)
    .execute(&mut *db)
    .await?;

    Ok(())
}

#[get("/task-tag?<task_id>&<tag_id>")]
async fn delete_task_tag(
    mut db: Connection<Db>,
    task_id: TaskID,
    tag_id: TagID,
) -> Result<(), TodoError> {
    let _result = sqlx::query(
        "
DELETE FROM tbl_task_tag WHERE task_id = ? AND tag_id = ?",
    )
    .bind(task_id)
    .bind(tag_id)
    .execute(&mut *db)
    .await?;

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
                create_reminder_task,
                create_tag,
                create_category,
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
        .mount(
            "/delete",
            routes![
                delete_tag,
                delete_category,
                delete_reminder_task,
                delete_duration_task,
                delete_common_task,
                delete_task_tag,
            ],
        )
        .mount(
            "/update",
            routes![
                convert_to_common_task,
                convert_to_duration_task,
                convert_to_reminder_task,
                update_duration_task,
                update_reminder_task,
                update_common_task,
                add_task_tag
            ],
        )
        .launch()
        .await?;
    Ok(())
}
