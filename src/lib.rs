use std::process; // used to exit process
use rusqlite::Connection;
mod models;
use models::{Task};

async fn create_task(conn: &Connection, new_task: &Task) {
    
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

fn parse_arguments(args: Vec<&str>, todo_list: &mut Vec<Task>, conn: &Connection) {
    let command = args.get(0);

    match command{
        Some(&"add") => {
        }
        Some(&"show") => {
        }
        Some(&"remove") => {
        }
        Some(&"update") => {
        }
        Some(&"toggle") => {
            if let Some(task_id) = args.get(1) {
                match task_id.parse::<u64>() {
                    Ok(task_id) => {
                        if let Ok(task) = get_task(todo_list, task_id) {
                            task.update_status();
                        } else {
                            println!("task not found in ToDo list")
                        }
                    } Err(e) => {
                        println!("{}", e)
                    }
                }
            }
        }
        Some(&"exit") => {
            process::exit(0)
        }
        Some(&"help") => {
            display_help();
        }
        _ => {
            println!("Input a valid command")
        }
    }
}

pub fn run(args: Vec<&str>, commands: &mut Vec<Task>, conn: &Connection) {
    parse_arguments(args, commands, conn);
}

// pub fn main() -> Task {
//     let tk = Task {
//         task: "damn".to_string(),
//         status: Status::Ongoing,
//         id: 64
//     };
//     println!("hello world {:?}",tk);
//     return tk;
// }