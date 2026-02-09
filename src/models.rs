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

#[derive(Debug)]
pub enum PriorityOrder {
    Low,
    Medium,
    High
}