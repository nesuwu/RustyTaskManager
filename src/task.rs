use serde::{Deserialize, Serialize};

/// Represents a single task in the to-do list.
#[derive(Serialize, Deserialize, Clone)]
pub struct Task {
    /// A unique identifier for the task.
    pub id: usize,
    /// The name or description of the task.
    pub name: String,
    /// Indicates whether the task is marked as done.
    pub done: bool,
}

/// Represents a collection of tasks.
#[derive(Serialize, Deserialize)]
pub struct Tasks {
    /// A vector containing all the tasks.
    pub tasks: Vec<Task>,
}
