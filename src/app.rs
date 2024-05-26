use serde::{Deserialize, Serialize};
use std::fs::{self, File};
use std::io::{self, Read};
use thiserror::Error;
use tui::widgets::ListState;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Task {
    pub id: usize,
    pub name: String,
    pub done: bool,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Tasks {
    pub tasks: Vec<Task>,
}

#[derive(PartialEq, Debug)]
pub enum AppMode {
    TaskMode,
    InputMode,
    EditMode,
}

pub struct App {
    pub tasks: Vec<Task>,
    pub state: ListState,
    pub mode: AppMode,
    pub input: String,
    pub next_id: usize,
    pub modify_index: Option<usize>,
}

impl App {
    pub fn new() -> App {
        App {
            tasks: Vec::new(),
            state: ListState::default(),
            mode: AppMode::TaskMode,
            input: String::new(),
            next_id: 0,
            modify_index: None,
        }
    }

    pub fn add_task(&mut self, name: String) {
        self.tasks.push(Task {
            id: self.next_id,
            name,
            done: false,
        });
        self.next_id += 1;
    }

    pub fn modify_task(&mut self, new_name: String) {
        if let Some(index) = self.modify_index {
            if let Some(task) = self.tasks.get_mut(index) {
                task.name = new_name;
            }
        }
    }

    pub fn toggle_task(&mut self) {
        if let Some(selected) = self.state.selected() {
            self.tasks[selected].done = !self.tasks[selected].done;
        }
    }

    pub fn save_tasks(&self) -> Result<(), AppError> {
        let tasks_data = Tasks { tasks: self.tasks.clone() };
        let toml_string = toml::to_string(&tasks_data).map_err(AppError::Serialization)?;
        fs::write("tasks.toml", toml_string).map_err(AppError::FileIO)?;
        Ok(())
    }

    pub fn load_tasks(&mut self) -> Result<(), AppError> {
        let mut file = match File::open("tasks.toml") {
            Ok(file) => file,
            Err(err) if err.kind() == io::ErrorKind::NotFound => return Ok(()), // File doesn't exist, so start with an empty list
            Err(err) => return Err(AppError::FileIO(err)),
        };
        let mut contents = String::new();
        file.read_to_string(&mut contents)?;

        let tasks_data: Tasks = toml::from_str(&contents).map_err(AppError::Deserialization)?;
        self.tasks = tasks_data.tasks;

        // Ensure the next ID is unique by finding the maximum existing ID
        self.next_id = self.tasks.iter().map(|task| task.id).max().unwrap_or(0) + 1;

        Ok(())
    }

    pub fn next_task(&mut self) {
        let i = match self.state.selected() {
            Some(i) => {
                if i >= self.tasks.len().saturating_sub(1) {
                    None
                } else {
                    Some(i + 1)
                }
            }
            None => Some(0),
        };
        self.state.select(i);
    }

    pub fn previous_task(&mut self) {
        let i = match self.state.selected() {
            Some(i) => {
                if i == 0 {
                    None
                } else {
                    Some(i - 1)
                }
            }
            None => Some(0),
        };
        self.state.select(i);
    }

    pub fn delete_task(&mut self) {
        if let Some(selected) = self.state.selected() {
            if selected < self.tasks.len() {
                self.tasks.remove(selected);
                self.state.select(Some(selected.saturating_sub(1)));
                self.save_tasks().ok(); // Save tasks after deletion (ignore errors for simplicity)
            }
        }
    }
}

#[derive(Error, Debug)]
pub enum AppError {
    #[error("Error reading or writing to file")]
    FileIO(#[from] io::Error),
    #[error("Error serializing tasks to TOML")]
    Serialization(#[from] toml::ser::Error),
    #[error("Error deserializing tasks from TOML")]
    Deserialization(#[from] toml::de::Error),
}