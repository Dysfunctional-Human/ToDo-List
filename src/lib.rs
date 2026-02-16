#![allow(dead_code)]
// #![allow(unused_imports)]
// used to exit process
// use std::process;   For exit command
// use rusqlite::{params, Connection};
mod models;
use models::{Task, TaskError, TaskStatus};
pub mod cli;
use cli::{Cli, Commands};
use clap::Parser;
mod db;
use db::{init_db, create_task, show_task_by_id, check_for_redundancy, get_tasks_by_status};

fn display_help() {
    let help: &str = "
    Welcome to ToDo list.
    Structure of query:
        command [Arguments]
    
    Supported Commands:
        help - Displays this help message
            usage: >help
    
    arguments:
    ";  

    println!("{}", help)
}

pub fn parse_arguments(args: Vec<&str>) -> Result<(), TaskError> {
    let mut clap_args = vec!["todo"];
    clap_args.extend(args);
    let cli = match Cli::try_parse_from(clap_args) {
        Ok(cli) => cli,
        Err(e) => {
            eprintln!("{}", e);
            return Ok(());  // Prevent crashing on bad input
        }
    };
    let conn = init_db()?;

    match cli.command {
        Commands::Add { title, priority, due, notes } => {
            let task_name = title.join(" ");
            let extras = notes.map(|n| n.join(" "));

            let task = Task {
                id: 0,
                title: task_name,
                status: TaskStatus::Ongoing,
                created_at: String::new(),
                updated_at: None,
                completed_at: None,
                deleted_at: None,
                due_at: due,
                priority: priority,
                notes: extras
            };
            check_for_redundancy(&conn, &task)?;
            let new_id = create_task(&conn, &task)?;
            let new_task = show_task_by_id(&conn, new_id)?;
            println!("Task added successfully: {}", new_task);
            Ok(())
        },
        Commands::Show { id } => {
            let task = show_task_by_id(&conn, id)?;
            println!("{}", task);
            Ok(())
        },
        Commands::List { all, completed, ongoing, low, medium, high, deleted} => {
            if completed || ongoing {
                let by_status = get_tasks_by_status(&conn, all, completed, ongoing)?;
                for task in by_status {
                    println!("{}", task)
                }
                Ok(())
            } else if low || medium || high {

            } else if all {

            } else if deleted {

            }
        }
    }
}