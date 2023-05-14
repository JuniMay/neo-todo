
use chrono::prelude::*;
use serde::{Deserialize, Serialize};

pub type TaskID = u32;
pub type Priority = u32;
pub type TagID = u32;
pub type CategoryID = u32;

pub trait Task {
    fn get_id(&self) -> TaskID;
    fn get_title(&self) -> String;
    fn get_deadline(&self) -> Option<DateTime<Utc>>;
    fn get_priority(&self) -> Option<Priority>;
    fn get_status(&self) -> Option<String>;
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct DurationTask {
    pub id: TaskID,
    pub title: String,
    
    pub description: Option<String>,
    pub deadline: Option<DateTime<Utc>>,
    pub priority: Option<Priority>,
    pub status: Option<String>,
    pub category_id: Option<CategoryID>,

    pub start_time: DateTime<Utc>,
    pub end_time: DateTime<Utc>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ReminderTask {
    pub id: TaskID,
    pub title: String,

    pub description: Option<String>,
    pub deadline: Option<DateTime<Utc>>,
    pub priority: Option<Priority>,
    pub status: Option<String>,
    pub category_id: Option<CategoryID>,

    pub remind_time: DateTime<Utc>,
}


#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct CommonTask {
    pub id: TaskID,
    pub title: String,

    pub description: Option<String>,
    pub deadline: Option<DateTime<Utc>>,
    pub priority: Option<Priority>,
    pub status: Option<String>,
    pub category_id: Option<CategoryID>,

    pub kind: u32,
}

impl Task for DurationTask {
    fn get_id(&self) -> TaskID {
        self.id
    }
    fn get_title(&self) -> String {
        self.title.clone()
    }
    fn get_deadline(&self) -> Option<DateTime<Utc>> {
        self.deadline
    }
    fn get_priority(&self) -> Option<Priority> {
        self.priority
    }
    fn get_status(&self) -> Option<String> {
        self.status.clone()
    }
}

impl Task for CommonTask {
    fn get_id(&self) -> TaskID {
        self.id
    }
    fn get_title(&self) -> String {
        self.title.clone()
    }
    fn get_deadline(&self) -> Option<DateTime<Utc>> {
        self.deadline
    }
    fn get_priority(&self) -> Option<Priority> {
        self.priority
    }
    fn get_status(&self) -> Option<String> {
        self.status.clone()
    }
}

impl Task for ReminderTask {
    fn get_id(&self) -> TaskID {
        self.id
    }
    fn get_title(&self) -> String {
        self.title.clone()
    }
    fn get_deadline(&self) -> Option<DateTime<Utc>> {
        self.deadline
    }
    fn get_priority(&self) -> Option<Priority> {
        self.priority
    }
    fn get_status(&self) -> Option<String> {
        self.status.clone()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct TaskDep {
    pub lhs_id: TaskID,
    pub rhs_id: TaskID,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Tag {
    pub id: TagID,
    pub name: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct TaskTag {
    pub task_id: TaskID,
    pub tag_id: TagID,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Category {
    pub id: CategoryID,
    pub name: String,
    pub description: String,
}