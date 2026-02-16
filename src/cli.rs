use clap::{Parser, Subcommand};
use crate::models::PriorityOrder;
#[derive(Parser)]
#[command(name="todo")]
#[command(about="A simple todo list application")]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands
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
        notes: Option<Vec<String>>
    },
    Show {
        /// Task id
        id: u64
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
        deleted: bool
    },
    Clear {},
    Exit {}
}