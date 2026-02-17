#![allow(dead_code)]
// #![allow(unused_imports)]
mod models;
use models::{Task, TaskError, TaskStatus};
pub mod cli;
use cli::{Cli, Commands};
use clap::Parser;
mod db;
use db::{init_db, create_task, show_task_by_id, check_for_redundancy, get_tasks_by_status, 
         get_tasks_by_priority, get_all_tasks, get_deleted_tasks, clear_screen, exit_app, update_status
    };

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
            } else if low || medium || high {
                let by_priority = get_tasks_by_priority(&conn, low, medium, high, all)?;
                for task in by_priority {
                    println!("{}", task)
                }
            } else if deleted {
                let deleted_tasks = get_deleted_tasks(&conn)?;
                for task in deleted_tasks {
                    println!("{}", task)
                }
            }  else if all {
                let all_tasks = get_all_tasks(&conn)?;
                for task in all_tasks {
                    println!("{}", task)
                }
            } else {
                let by_status = get_tasks_by_status(&conn, false, false, true)?;
                for task in by_status {
                    println!("{}", task)
                }
            }
            Ok(())
        },
        Commands::Done { id } => {
            update_status(&conn, id, TaskStatus::Completed)?;
            let new_task = show_task_by_id(&conn, id)?;
            println!("Updated Task: {}", new_task);
            Ok(())
        },
        Commands::Reopen { id } => {
            update_status(&conn, id, TaskStatus::Ongoing)?;
            let new_task = show_task_by_id(&conn, id)?;
            println!("Updated Task: {}", new_task);
            Ok(())
        },
        Commands::Clear {} => { 
            clear_screen();
            Ok(())
        },
        Commands::Exit {} => { 
            exit_app();
            Ok(())
        }
    }
}