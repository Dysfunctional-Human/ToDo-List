// #![allow(unused_variables)]
// #![allow(unused_imports)]
#![allow(dead_code)]
use clap::ValueEnum;
use core::fmt;

#[derive(Debug)]
pub struct Task {
    pub id: u64,
    pub title: String,
    pub status: TaskStatus,
    pub created_at: String,
    pub updated_at: Option<String>,
    pub completed_at: Option<String>,
    pub deleted_at: Option<String>,
    pub due_at: Option<String>,
    pub priority: Option<PriorityOrder>,
    pub notes: Option<String>
}

impl fmt::Display for Task {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Task #{}: {} [{}] - Created: {}{}{}",
            self.id,
            self.title,
            self.status.as_str(),
            self.created_at,
            self.priority.as_ref().map_or(String::new(), |p| format!(" | Priority: {}", p.as_str())),
            self.notes.as_ref().map_or(String::new(), |n| format!(" | Notes: {}", n))
        )
    }
}

#[derive(Debug, Clone, ValueEnum)]
pub enum TaskStatus {
    Ongoing,
    Completed
}

impl TaskStatus {
    pub fn as_str(&self) -> &str {
        match self {
            TaskStatus::Ongoing => "Ongoing",
            TaskStatus::Completed => "Completed"
        }
    }
}

#[derive(Debug, Clone, ValueEnum)]
pub enum PriorityOrder {
    Low,
    Medium,
    High
}

impl PriorityOrder {
    pub fn as_str(&self) -> &str {
        match self {
            PriorityOrder::High => "High",
            PriorityOrder::Medium => "Medium",
            PriorityOrder::Low => "Low"
        }
    }
}

#[derive(Debug)]
pub enum TaskError {
    DatabaseError(rusqlite::Error),
    InvalidInput(String),
    DuplicateTask(String)
}

impl From<rusqlite::Error> for TaskError {
    fn from(err: rusqlite::Error) -> TaskError {
        TaskError::DatabaseError(err)
    }
}