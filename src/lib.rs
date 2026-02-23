use clap::Parser;
mod cli;
mod models;
pub mod db;
use crate::{
    cli::{Cli, Commands}, db::{check_for_redundancy, check_task_exists_by_id, clear_screen, create_task, exit_app, get_all_tasks, get_deleted_tasks, get_due_tasks, get_tasks_by_priority, get_tasks_by_status, init_db, purge_task, restore_task, show_task_by_id, soft_delete_task, update_status, update_task_by_id
    }, models::{Task, TaskError, TaskStatus}
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
                for (i, task) in by_status.iter().enumerate() {
                    println!("{}. {}", i+1, task)
                }
            } else if low || medium || high {
                let by_priority = get_tasks_by_priority(&conn, low, medium, high, all)?;
                for (i, task) in by_priority.iter().enumerate() {
                    println!("{}. {}", i+1, task)
                }
            } else if deleted {
                let deleted_tasks = get_deleted_tasks(&conn)?;
                for (i, task) in deleted_tasks.iter().enumerate() {
                    println!("{}. {}", i+1, task)
                }
            }  else if all {
                let all_tasks = get_all_tasks(&conn)?;
                for (i, task) in all_tasks.iter().enumerate() {
                    println!("{}. {}", i+1, task)
                }
            } else {
                let by_status = get_tasks_by_status(&conn, false, false, true)?;
                for (i, task) in by_status.iter().enumerate() {
                    println!("{}. {}", i+1, task)
                }
            }
            Ok(())
        },
        Commands::Done { id } => {
            check_task_exists_by_id(&conn, id)?;
            update_status(&conn, id, TaskStatus::Completed)?;
            let new_task = show_task_by_id(&conn, id)?;
            println!("Updated Task: {}", new_task);
            Ok(())
        },
        Commands::Reopen { id } => {
            check_task_exists_by_id(&conn, id)?;
            update_status(&conn, id, TaskStatus::Ongoing)?;
            let new_task = show_task_by_id(&conn, id)?;
            println!("Updated Task: {}", new_task);
            Ok(())
        },
        Commands::Delete { id } => {
            check_task_exists_by_id(&conn, id)?;
            soft_delete_task(&conn, id)?;
            println!("Task deleted successfully");
            Ok(())
        },
        Commands::Restore { id } => {
            check_task_exists_by_id(&conn, id)?;
            restore_task(&conn, id)?;
            let task = show_task_by_id(&conn, id)?;
            println!("Restored Task: {}", task);
            Ok(())
        },
        Commands::Purge { id, all } => {
            purge_task(&conn, id, all)?;
            println!("Task(s) permanently deleted");
            Ok(())
        },
        Commands::Due { today, tomorrow } => {
            let due_tasks = get_due_tasks(&conn, today, tomorrow)?;
            for (i, task) in due_tasks.iter().enumerate() {
                println!("{}. {}", i+1, task)
            }
            Ok(())
        },
        Commands::Update { id, title, due, priority, notes } => {
            check_task_exists_by_id(&conn, id)?;
            let new_title = title.map(|n| n.join(" "));
            let new_notes = notes.map(|n| n.join(" "));

            update_task_by_id(&conn, id, new_title, due, priority, new_notes)?;
            let updated_task = show_task_by_id(&conn, id)?;
            println!("Update Successful! Task: {}", updated_task);
            Ok(())
        },
        Commands::Help {  } => {
            display_help();
            Ok(())
        },
        Commands::Clear {} => { 
            clear_screen();
            Ok(())
        },
        Commands::Exit {} => {
            println!("Bye Bye...👋"); 
            exit_app();
            Ok(())
        }
        // ToDo - 1. Updated_at for the commands, completed_at for the commands
    }
}