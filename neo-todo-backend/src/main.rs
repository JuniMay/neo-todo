#[macro_use]
extern crate rocket;

use neo_todo::{CommonTask, TaskID};
use rocket::serde::json::Json;
use rocket_db_pools::{
    sqlx::{self, Row},
    Connection, Database,
};

#[derive(Database)]
#[database("todo_db")]
struct Db(sqlx::MySqlPool);

#[get("/")]
async fn index(mut db: Connection<Db>) -> Json<Vec<(TaskID, String, Option<String>)>> {
    let result = sqlx::query(
        "
SELECT task_id, task_title, task_status FROM tbl_task",
    )
    .fetch_all(&mut *db)
    .await
    .map_or(Json(Vec::new()), |v| {
        let mut tasks = Vec::new();
        for row in v {
            tasks.push((row.get(0), row.get(1), row.try_get(2).ok()));
        }
        Json(tasks)
    });
    result
}

#[post("/common-task", data = "<task>")]
async fn create_common_task(mut db: Connection<Db>, task: Json<CommonTask>) {
    let common_task = task.into_inner();
    let id = common_task.id;
    let title = common_task.title.clone();
    let description = common_task.description.clone();
    let deadline = common_task.deadline;
    let priority = common_task.priority;
    let status = common_task.status;
    let category_id = common_task.category_id;

    let _ = sqlx::query(
        "
INSERT INTO tbl_task (
    task_id, 
    task_title, 
    task_description, 
    task_deadline, 
    task_priority, 
    task_status, 
    category_id
) VALUES (?, ?, ?, ?, ?, ?, ?)",
    )
    .bind(id)
    .bind(title)
    .bind(description)
    .bind(deadline)
    .bind(priority)
    .bind(status)
    .bind(category_id)
    .execute(&mut *db)
    .await;
}

#[rocket::main]
async fn main() -> anyhow::Result<()> {
    let rocket = rocket::build();
    rocket
        .attach(Db::init())
        .mount("/", routes![index])
        .mount("/create", routes![create_common_task])
        .mount("/fetch", routes![])
        .launch()
        .await?;
    Ok(())
}
