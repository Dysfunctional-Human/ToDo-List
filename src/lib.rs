#![allow(dead_code)]
#![allow(unused_imports)]
use std::process; 
use chrono::Utc;
// used to exit process
use rusqlite::{params, Connection};
mod models;
use models::{Task, TaskError, TaskStatus,};
pub mod cli;
use cli::{Cli, Commands};
use clap::Parser;
mod db;
use db::init_db;

fn create_task(
    conn: &Connection,
    new_task: &Task
) -> Result<(), TaskError> {
    let priority = new_task.priority.as_ref().map(|p| p.as_str());
    let now = Utc::now().format("%d/%m/%Y").to_string();

    conn.execute(
        "INSERT INTO tasks (title, status, created_at, due_at,
                            priority, notes)
         VALUES (?1, ?2, ?3, ?4, ?5, ?6)
        "
        , params![&new_task.title, &new_task.status.as_str(), now, 
                  &new_task.due_at, priority, &new_task.notes]
    )?;

    Ok(())
}

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

// fn parse_arguments(args: Vec<&str>, conn: &Connection) {
//     let command = args.get(0);

//     match command{
//         Some(&"add") => {
//             if args.len() > 1 {
//                 let task_data = args[1..].join(" ");
//                 let new_task = Task{
//                     id: 0,
//                     title: task_data,
//                     status: TaskStatus::Ongoing,
//                     created_at: Utc::now().format("%d/%m/%Y").to_string(),
//                     priority: None,
//                     notes: None,
//                     updated_at: None,
//                     completed_at: None,
//                     deleted_at: None,
//                     due_at: None
//                 };

//                 match create_task(conn, &new_task) {
//                     // Todo, also display the new task after creation
//                     Ok(()) => println!("Task Created Successfully"),
//                     Err(e) => println!("Failed to create task: {:?}", e)
//                 }
//             }
//         }
//         Some(&"show") => {
//         }
//         Some(&"remove") => {
//         }
//         Some(&"update") => {
//         }
//         Some(&"toggle") => {
//         }
//         Some(&"exit") => {
//             process::exit(0)
//         }
//         Some(&"help") => {
//             display_help();
//         }
//         _ => {
//             println!("Input a valid command")
//         }
//     }
// }

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
        Commands::Add { title, priority, due_at, notes } => {
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
                due_at: due_at,
                priority: priority,
                notes: extras
            };

            create_task(&conn, &task)?;
            println!("Task added successfuly");
            // Todo display created task here after creation
            Ok(())
        }
    }
}

// pub fn run() {
//     parse_arguments();
// }

// pub fn main() -> Task {
//     let tk = Task {
//         task: "damn".to_string(),
//         status: Status::Ongoing,
//         id: 64
//     };
//     println!("hello world {:?}",tk);
//     return tk;
// }