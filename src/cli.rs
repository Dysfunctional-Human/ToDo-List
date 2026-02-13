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
    }
}