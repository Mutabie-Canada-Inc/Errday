use crate::models::{Task, Quadrant, TaskStatus};
use dioxus::prelude::*;
use directories::ProjectDirs;
use std::fs;
use std::path::PathBuf;

// Application metadata used to determine where to store data on the user's computer
const APP_QUALIFIER: &str = "ca";
const APP_ORG: &str = "mutabie";
const APP_NAME: &str = "errday";
const DATA_FILE: &str = "tasks.json";

/// APP STATE: The central hub for all application data
#[derive(Clone, Copy)]
pub struct AppState {
    pub tasks: Signal<Vec<Task>>, // A reactive list of tasks that updates the UI automatically
}

impl AppState {
    /// INITIALIZATION: Starts the state by loading saved tasks from the disk
    pub fn new() -> Self {
        let tasks = Self::load_tasks();
        Self {
            tasks: Signal::new(tasks),
        }
    }

    /// DATA LOCATION: Finds or creates the folder where we save our mission data
    fn get_data_path() -> PathBuf {
        // We use standard platform-specific directories (e.g., Application Support on macOS)
        if let Some(proj_dirs) = ProjectDirs::from(APP_QUALIFIER, APP_ORG, APP_NAME) {
            let data_dir = proj_dirs.data_dir();
            // Ensure the directory exists before we try to write to it
            if !data_dir.exists() {
                let _ = fs::create_dir_all(data_dir);
            }
            return data_dir.join(DATA_FILE);
        }
        PathBuf::from(DATA_FILE) // Fallback to the current directory if we can't find the home folder
    }

    /// LOADING DATA: Reads the tasks from the JSON file on startup
    fn load_tasks() -> Vec<Task> {
        let path = Self::get_data_path();
        if path.exists() {
            if let Ok(content) = fs::read_to_string(path) {
                // Turn the JSON text back into a list of Rust Task objects
                if let Ok(tasks) = serde_json::from_str(&content) {
                    return tasks;
                }
            }
        }
        Vec::new() // Start with an empty list if no file exists yet
    }

    /// SAVING DATA: Writes the current list of tasks to the disk
    pub fn save_tasks(&self) {
        let path = Self::get_data_path();
        let tasks = self.tasks.read();
        // Convert our list of tasks into formatted JSON text
        if let Ok(content) = serde_json::to_string_pretty(&*tasks) {
            let _ = fs::write(path, content);
        }
    }

    /// ACTION - ADD TASK: Adds a new task to the inbox
    pub fn add_task(&self, title: String) {
        let mut tasks_sig = self.tasks;
        let mut tasks = tasks_sig.write();
        tasks.push(Task::new(title));
        drop(tasks); // Release the write lock
        self.save_tasks(); // Persist changes immediately
    }

    /// ACTION - SORT TASK: Moves a task to a different matrix quadrant
    pub fn update_task_quadrant(&self, id: uuid::Uuid, quadrant: Quadrant) {
        let mut tasks_sig = self.tasks;
        let mut tasks = tasks_sig.write();
        if let Some(task) = tasks.iter_mut().find(|t| t.id == id) {
            task.quadrant = quadrant;
        }
        drop(tasks);
        self.save_tasks();
    }

    /// ACTION - TOGGLE STATUS: Marks a task as Done or Todo
    #[allow(dead_code)]
    pub fn toggle_task_status(&self, id: uuid::Uuid) {
        let mut tasks_sig = self.tasks;
        let mut tasks = tasks_sig.write();
        if let Some(task) = tasks.iter_mut().find(|t| t.id == id) {
            task.status = match task.status {
                TaskStatus::Todo => TaskStatus::Done,
                TaskStatus::Done => TaskStatus::Todo,
            };
        }
        drop(tasks);
        self.save_tasks();
    }
    
    /// ACTION - DELETE: Permanently removes a task
    pub fn delete_task(&self, id: uuid::Uuid) {
        let mut tasks_sig = self.tasks;
        let mut tasks = tasks_sig.write();
        tasks.retain(|t| t.id != id);
        drop(tasks);
        self.save_tasks();
    }
    
    /// ACTION - SCHEDULE: Sets the time block for a task on the calendar
    pub fn update_task_schedule(&self, id: uuid::Uuid, start: Option<chrono::DateTime<chrono::Local>>, end: Option<chrono::DateTime<chrono::Local>>) {
        let mut tasks_sig = self.tasks;
        let mut tasks = tasks_sig.write();
        if let Some(task) = tasks.iter_mut().find(|t| t.id == id) {
            task.scheduled_start = start;
            task.scheduled_end = end;
        }
        drop(tasks);
        self.save_tasks();
    }
}
