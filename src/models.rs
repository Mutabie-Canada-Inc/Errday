use chrono::{DateTime, Local};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// THE MATRIX SECTORS: Each task belongs to one of these quadrants
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Quadrant {
    DoFirst,  // Urgent & Important: Immediate action required
    Schedule, // Important, but not Urgent: Defer to a specific time
    Delegate, // Urgent, but not Important: Pass to someone else
    Delete,   // Neither Urgent nor Important: Remove from mission
    Unsorted, // The Inbox: Newly captured tasks waiting for sorting
}

/// OPERATIONAL STATUS: Is the task active or archived?
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum TaskStatus {
    Todo,     // Action pending
    Done,     // Mission accomplished
}

/// MISSION TASK: The core unit of data in Errday
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Task {
    pub id: Uuid,                            // Unique identifier for each task
    pub title: String,                       // The name of the task
    pub description: Option<String>,         // Extra details (optional)
    pub quadrant: Quadrant,                  // Where the task sits in the Eisenhower Matrix
    pub status: TaskStatus,                  // Whether it is finished or not
    pub created_at: DateTime<Local>,         // The exact moment the task was created
    pub scheduled_start: Option<DateTime<Local>>, // Planned start time for the calendar
    pub scheduled_end: Option<DateTime<Local>>,   // Planned completion time
}

impl Task {
    /// CREATING A NEW TASK: Generates a task with default values (Unsorted, Todo)
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
