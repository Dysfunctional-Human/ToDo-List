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
    pub extra_notes: Option<String>
}

#[derive(Debug)]
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

#[derive(Debug)]
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
    InvalidInput(String)
}

impl From<rusqlite::Error> for TaskError {
    fn from(err: rusqlite::Error) -> TaskError {
        TaskError::DatabaseError(err)
    }
}