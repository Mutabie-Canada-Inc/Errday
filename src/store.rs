use crate::models::{Task, Quadrant, TaskStatus};
use dioxus::prelude::*;
use directories::ProjectDirs;
use std::fs;
use std::path::PathBuf;

const APP_QUALIFIER: &str = "ca";
const APP_ORG: &str = "mutabie";
const APP_NAME: &str = "errday";
const DATA_FILE: &str = "tasks.json";

#[derive(Clone, Copy)]
pub struct AppState {
    pub tasks: Signal<Vec<Task>>,
}

impl AppState {
    pub fn new() -> Self {
        let tasks = Self::load_tasks();
        Self {
            tasks: Signal::new(tasks),
        }
    }

    fn get_data_path() -> PathBuf {
        if let Some(proj_dirs) = ProjectDirs::from(APP_QUALIFIER, APP_ORG, APP_NAME) {
            let data_dir = proj_dirs.data_dir();
            if !data_dir.exists() {
                let _ = fs::create_dir_all(data_dir);
            }
            return data_dir.join(DATA_FILE);
        }
        PathBuf::from(DATA_FILE) // Fallback to current directory
    }

    fn load_tasks() -> Vec<Task> {
        let path = Self::get_data_path();
        if path.exists() {
            if let Ok(content) = fs::read_to_string(path) {
                if let Ok(tasks) = serde_json::from_str(&content) {
                    return tasks;
                }
            }
        }
        Vec::new()
    }

    pub fn save_tasks(&self) {
        let path = Self::get_data_path();
        let tasks = self.tasks.read();
        if let Ok(content) = serde_json::to_string_pretty(&*tasks) {
            let _ = fs::write(path, content);
        }
    }

    pub fn add_task(&self, title: String) {
        let mut tasks_sig = self.tasks;
        let mut tasks = tasks_sig.write();
        tasks.push(Task::new(title));
        drop(tasks);
        self.save_tasks();
    }

    pub fn update_task_quadrant(&self, id: uuid::Uuid, quadrant: Quadrant) {
        let mut tasks_sig = self.tasks;
        let mut tasks = tasks_sig.write();
        if let Some(task) = tasks.iter_mut().find(|t| t.id == id) {
            task.quadrant = quadrant;
        }
        drop(tasks);
        self.save_tasks();
    }

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
    
    pub fn delete_task(&self, id: uuid::Uuid) {
        let mut tasks_sig = self.tasks;
        let mut tasks = tasks_sig.write();
        tasks.retain(|t| t.id != id);
        drop(tasks);
        self.save_tasks();
    }
}
