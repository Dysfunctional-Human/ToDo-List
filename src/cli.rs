use crate::models::PriorityOrder;
use clap::{Parser, Subcommand};
#[derive(Parser)]
#[command(name = "todo")]
#[command(about = "A simple todo list application")]
#[command(disable_help_subcommand = true)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Add a new task
    Add {
        /// The task title
        title: Vec<String>,

        /// Priority of task
        #[arg(long, value_enum)]
        priority: Option<PriorityOrder>,

        /// Due date in dd/mm/yyyy
        #[arg(long)]
        due: Option<String>,

        /// Extra notes
        #[arg(long, num_args = 1..)]
        notes: Option<Vec<String>>,
    },
    Show {
        /// Task id
        id: u64,
    },
    List {
        /// Show all tasks including deleted
        #[arg(long)]
        all: bool,

        /// Show only completed tasks
        #[arg(long, conflicts_with_all = ["ongoing", "low", "medium", "high", "deleted"])]
        completed: bool,

        /// Show only Ongoing tasks
        #[arg(long, conflicts_with_all = ["completed", "low", "medium", "high", "deleted"])]
        ongoing: bool,

        /// Show only low priority tasks
        #[arg(long, conflicts_with_all = ["completed", "ongoing", "medium", "high", "deleted"])]
        low: bool,

        /// Show only medium priority tasks
        #[arg(long, conflicts_with_all = ["completed", "ongoing", "low", "high", "deleted"])]
        medium: bool,

        /// Show only high priority tasks
        #[arg(long, conflicts_with_all = ["completed", "ongoing", "medium", "low", "deleted"])]
        high: bool,

        /// Show only deleted tasks
        #[arg(long, conflicts_with_all = ["completed", "ongoing", "medium", "high"])]
        deleted: bool,
    },
    Done {
        /// Task id
        id: u64,
    },
    Reopen {
        /// Task id
        id: u64,
    },
    Delete {
        /// Task id
        id: u64,
    },
    Restore {
        /// Task id
        id: u64,
    },
    Purge {
        // Permanently delete task by id (works only on soft-deleted tasks)
        id: Option<u64>,

        // Permanently delete all soft deleted tasks
        #[arg(long, conflicts_with_all=["id"])]
        all: bool,
    },
    Due {
        /// Show tasks due today
        #[arg(long)]
        today: bool,

        /// Show tasks due tomorrow
        #[arg(long)]
        tomorrow: bool,
    },
    Update {
        /// id
        id: u64,

        /// title
        #[arg(long, num_args = 1..)]
        title: Option<Vec<String>>,

        /// due
        #[arg(long)]
        due: Option<String>,

        /// priority
        #[arg(long, value_enum)]
        priority: Option<PriorityOrder>,

        /// notes
        #[arg(long, num_args = 1..)]
        notes: Option<Vec<String>>,
    },
    Search {
        /// keyword(s)
        search_string: Vec<String>,
    },
    #[command(hide = true)]
    Seed {
        /// Clear existing tasks before seeding
        #[arg(long)]
        reset: bool,
    },
    Stats {},
    Help {},
    Clear {},
    #[command(alias = "quit")]
    Exit {},
}
