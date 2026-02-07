use chrono::{DateTime, Local};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Quadrant {
    DoFirst,  // Urgent & Important
    Schedule, // Not Urgent & Important
    Delegate, // Urgent & Not Important
    Delete,   // Not Urgent & Not Important
    Unsorted, // Inbox
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum TaskStatus {
    Todo,
    Done,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Task {
    pub id: Uuid,
    pub title: String,
    pub description: Option<String>,
    pub quadrant: Quadrant,
    pub status: TaskStatus,
    pub created_at: DateTime<Local>,
    pub scheduled_start: Option<DateTime<Local>>,
    pub scheduled_end: Option<DateTime<Local>>, // For time blocking
}

impl Task {
    pub fn new(title: String) -> Self {
        Self {
            id: Uuid::new_v4(),
            title,
            description: None,
            quadrant: Quadrant::Unsorted,
            status: TaskStatus::Todo,
            created_at: Local::now(),
            scheduled_start: None,
            scheduled_end: None,
        }
    }
}
