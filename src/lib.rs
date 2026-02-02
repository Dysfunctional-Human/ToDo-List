use std::{
    process::{self},    // used to exit process
    sync::atomic::{self, AtomicU64}     // thread safe counter
};

#[derive(Debug)]
pub enum Status {
    Ongoing,
    Completed
}

#[derive(Debug)]
pub struct Task {
    task: String,   
    status: Status,   
    id: u64
}

impl Task{
    fn update_status(&mut self) {
        self.status = match self.status{
            Status::Completed => Status::Ongoing,
            Status::Ongoing => Status::Completed
        };
    }

    fn update_task(&mut self, new_name: impl Into<String>) {
        self.task = new_name.into();
    }
}

fn display_todo(todo_list: &Vec<Task>) {
    if todo_list.len() < 1 {
        println!("Empty todo list");
        return;
    }

    for item in todo_list {
        println!("
            {}. {} - status: {:?}
        ", item.id, item.task, item.status)
    }
}

static UNIQUE_ID: AtomicU64 = AtomicU64::new(1);
fn add_new_task(todo_list: &mut Vec<Task>, task_name: &str) {

    let task = Task {
        task: task_name.into(),
        status: Status::Ongoing,
        id: UNIQUE_ID.fetch_add(1, atomic::Ordering::SeqCst)
        // Automatically increments the counter, returns old value and guarantees uniqueness
    };

    todo_list.push(task);

    println!("ToDo list updated")
}

fn remove_task(todo_list: &mut Vec<Task>, id_no: u64) {
    todo_list.retain(|task| task.id != id_no); // Keeps those tasks that don't match given id
}

fn get_task(todo_list: &mut Vec<Task>, id_no: u64) -> Result<&mut Task, &str> {
    for task in todo_list {
        if task.id == id_no {
            return Ok(task)
        }
    }
    return Err("Error: Couldn't find task with associated Id");
}

fn display_help() {
    let help: &str = "
    Welcome to ToDo list.
    Structure of query:
        command [Arguments]
    
    Supported Commands:
        add - Add a new task to the ToDo list
            usage: >add task_string
        
        show - Display the ToDo list
            usage: >show 
        
        remove - Removes a task from the ToDo list
            usage: >remove task_id
        
        update - Change the name of a task
            usage: >update task_id new_task_string
        
        toggle - Toggle the status between Ongoing, Completed
            usage: >toggle task_id

        exit - Exit the program
            usage: >exit
        
        help - Displays this help message
            usage: >help
    
    arguments:
        task_id: The unique id assigned to each task

        task_string: The string for the task provided by the user. 
    ";  

    println!("{}", help)
}

fn parse_arguments(args: Vec<&str>, todo_list: &mut Vec<Task>) {
    let command = args.get(0);

    match command{
        Some(&"add") => {
            if args.len() > 1 {
                let new_todo = args[1..].join(" ");
                add_new_task(todo_list, &new_todo);
                display_todo(todo_list);
            } else {
                println!("Please provide the task string")
            }
        }
        Some(&"show") => {
            display_todo(todo_list);
        }
        Some(&"remove") => {
            if let Some(value) = args.get(1) {
                match value.parse::<u64>() {
                    Ok(value) => {
                        remove_task(todo_list, value);
                        println!("ToDO List updated")
                    }
                    Err(e) => {
                        println!("{}", e.to_string())
                    }
                }
            }
        }
        Some(&"update") => {
            if let Some(task_id) = args.get(1) {
                match task_id.parse::<u64>() {
                    Ok(task_id) => {
                        if let Ok(task) = get_task(todo_list, task_id) {
                            if args.len() > 2 {
                                let updated_string = args[2..].join(" ");
                                task.update_task(updated_string.to_string());
                            } else {
                                println!("No new string provided")
                            }
                        } else {
                            println!("task not found in ToDo list")
                        }
                    } Err(e) => {
                        println!("{}", e)
                    }
                }
            }
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

pub fn run(args: Vec<&str>, commands: &mut Vec<Task>) {
    parse_arguments(args, commands);
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