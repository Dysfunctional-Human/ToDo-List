use clap::Parser;
use rusqlite::Connection;
mod cli;
pub mod db;
mod models;
mod seed;
use crate::{
    cli::{Cli, Commands},
    db::{
        check_for_redundancy, check_task_exists_by_id, clear_screen, create_task, exit_app,
        get_all_tasks, get_deleted_tasks, get_due_tasks, get_stats, get_tasks_by_priority,
        get_tasks_by_status, purge_task, restore_task, search_by_string, show_task_by_id,
        soft_delete_task, update_status, update_task_by_id, validate_date_format,
    },
    models::{Task, TaskError, TaskStatus},
    seed::seed_database,
};

fn display_help() {
    let help = r#"
╔══════════════════════════════════════════════════════════════════════════════╗
║                            ToDo List - Help                                  ║
╚══════════════════════════════════════════════════════════════════════════════╝

USAGE: command [arguments] [options]

────────────────────────────────────────────────────────────────────────────────
ADDING & VIEWING TASKS
────────────────────────────────────────────────────────────────────────────────

  add <title> [options]       Add a new task
      --priority <PRIORITY>   Set priority: low, medium, high
      --due <DATE>            Set due date (dd/mm/yyyy)
      --notes <TEXT>          Add notes (supports multiple words)

      Examples:
        add Buy groceries
        add Doctor appointment --priority high --due 25/02/2026
        add Submit report --notes remember to include charts --priority medium

  show <id>                   Display detailed view of a single task
      Example: show 5

  list [options]              List tasks (defaults to ongoing tasks)
      --all                   Show all tasks (excluding deleted)
      --completed             Show only completed tasks
      --ongoing               Show only ongoing tasks
      --low                   Show only low priority tasks
      --medium                Show only medium priority tasks
      --high                  Show only high priority tasks
      --deleted               Show only deleted tasks
      --all --deleted         Show everything including deleted

      Examples:
        list                  (shows ongoing tasks)
        list --completed
        list --high
        list --all --deleted

  search <keywords>           Search tasks by title or notes
      Example: search doctor appointment

────────────────────────────────────────────────────────────────────────────────
MANAGING TASK STATUS
────────────────────────────────────────────────────────────────────────────────

  done <id>                   Mark a task as completed
      Example: done 3

  reopen <id>                 Reopen a completed task (set back to ongoing)
      Example: reopen 3

────────────────────────────────────────────────────────────────────────────────
UPDATING TASKS
────────────────────────────────────────────────────────────────────────────────

  update <id> [options]       Update task fields (all options are optional)
      --title <TEXT>          Change the title
      --due <DATE>            Change the due date (dd/mm/yyyy)
      --priority <PRIORITY>   Change priority: low, medium, high
      --notes <TEXT>          Change the notes

      Examples:
        update 5 --priority high
        update 5 --title New task title --due 01/03/2026
        update 5 --notes updated notes here

────────────────────────────────────────────────────────────────────────────────
DELETING & RESTORING TASKS
────────────────────────────────────────────────────────────────────────────────

  delete <id>                 Soft delete a task (can be restored)
      Example: delete 7

  restore <id>                Restore a soft-deleted task
      Example: restore 7

  purge <id>                  Permanently delete a specific task
  purge --all                 Permanently delete ALL soft-deleted tasks
      Examples:
        purge 7
        purge --all

────────────────────────────────────────────────────────────────────────────────
DUE DATES & STATISTICS
────────────────────────────────────────────────────────────────────────────────

  due [options]               Show tasks by due date
      --today                 Tasks due today
      --tomorrow              Tasks due tomorrow

      Examples:
        due --today
        due --tomorrow

  stats                       Display task statistics
                              (total, ongoing, completed, by priority)

────────────────────────────────────────────────────────────────────────────────
UTILITY COMMANDS
────────────────────────────────────────────────────────────────────────────────

  help                        Display this help message
  clear                       Clear the screen
  exit, quit                  Exit the application

────────────────────────────────────────────────────────────────────────────────
NOTES
────────────────────────────────────────────────────────────────────────────────
  - Date format: dd/mm/yyyy (e.g., 25/02/2026)
  - Priority values: low, medium, high (lowercase)
  - Multi-word titles/notes don't need quotes
  - Task IDs can be found using 'list' or 'search'

"#;

    println!("{}", help)
}

pub fn parse_arguments(conn: &Connection, args: Vec<&str>) -> Result<(), TaskError> {
    let mut clap_args = vec!["todo"];
    clap_args.extend(args);
    let cli = match Cli::try_parse_from(clap_args) {
        Ok(cli) => cli,
        Err(e) => {
            let error_text = e.to_string().replace(
                "For more information, try '--help'.",
                "For more information, type 'help'.",
            );
            eprintln!("{}", error_text);
            return Ok(()); // Prevent crashing on bad input
        }
    };

    match cli.command {
        Commands::Add {
            title,
            priority,
            due,
            notes,
        } => {
            let task_name = title.join(" ");
            if task_name.trim().is_empty() {
                return Err(TaskError::InvalidInput(
                    "Task title cannot be empty".to_string(),
                ));
            }
            let extras = notes.map(|n| n.join(" "));
            if let Some(ref due_date) = due {
                validate_date_format(&due_date)?;
            }

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
                notes: extras,
            };
            check_for_redundancy(&conn, &task)?;
            let new_id = create_task(&conn, &task)?;
            let new_task = show_task_by_id(&conn, new_id)?;
            println!("Task added successfully: {}", new_task);
            Ok(())
        }
        Commands::Show { id } => {
            let task = show_task_by_id(&conn, id)?;
            println!("{}", task);
            Ok(())
        }
        Commands::List {
            all,
            completed,
            ongoing,
            low,
            medium,
            high,
            deleted,
        } => {
            if completed || ongoing {
                let by_status = get_tasks_by_status(&conn, all, completed, ongoing)?;
                if by_status.is_empty() {
                    return Err(TaskError::NoTaskFound("No task found".to_string()));
                }
                for (i, task) in by_status.iter().enumerate() {
                    println!("{}. {}", i + 1, task)
                }
            } else if low || medium || high {
                let by_priority = get_tasks_by_priority(&conn, low, medium, high, all)?;
                if by_priority.is_empty() {
                    return Err(TaskError::NoTaskFound("No task found".to_string()));
                }
                for (i, task) in by_priority.iter().enumerate() {
                    println!("{}. {}", i + 1, task)
                }
            } else if deleted {
                let deleted_tasks = get_deleted_tasks(&conn, all)?;
                if deleted_tasks.is_empty() {
                    return Err(TaskError::NoTaskFound("No task found".to_string()));
                }
                for (i, task) in deleted_tasks.iter().enumerate() {
                    println!("{}. {}", i + 1, task)
                }
            } else if all {
                let all_tasks = get_all_tasks(&conn)?;
                if all_tasks.is_empty() {
                    return Err(TaskError::NoTaskFound("No task found".to_string()));
                }
                for (i, task) in all_tasks.iter().enumerate() {
                    println!("{}. {}", i + 1, task)
                }
            } else {
                let by_status = get_tasks_by_status(&conn, false, false, true)?;
                if by_status.is_empty() {
                    return Err(TaskError::NoTaskFound("No task found".to_string()));
                }
                for (i, task) in by_status.iter().enumerate() {
                    println!("{}. {}", i + 1, task)
                }
            }
            Ok(())
        }
        Commands::Done { id } => {
            check_task_exists_by_id(&conn, id)?;
            update_status(&conn, id, TaskStatus::Completed)?;
            let new_task = show_task_by_id(&conn, id)?;
            println!("Updated Task: {}", new_task);
            Ok(())
        }
        Commands::Reopen { id } => {
            check_task_exists_by_id(&conn, id)?;
            update_status(&conn, id, TaskStatus::Ongoing)?;
            let new_task = show_task_by_id(&conn, id)?;
            println!("Updated Task: {}", new_task);
            Ok(())
        }
        Commands::Delete { id } => {
            check_task_exists_by_id(&conn, id)?;
            soft_delete_task(&conn, id)?;
            println!("Task deleted successfully");
            Ok(())
        }
        Commands::Restore { id } => {
            check_task_exists_by_id(&conn, id)?;
            restore_task(&conn, id)?;
            let task = show_task_by_id(&conn, id)?;
            println!("Restored Task: {}", task);
            Ok(())
        }
        Commands::Purge { id, all } => {
            purge_task(&conn, id, all)?;
            println!("Task(s) permanently deleted");
            Ok(())
        }
        Commands::Due { today, tomorrow } => {
            let due_tasks = get_due_tasks(&conn, today, tomorrow)?;
            if due_tasks.is_empty() {
                return Err(TaskError::NoTaskFound(
                    "No tasks found with the given due date".to_string(),
                ));
            }
            for (i, task) in due_tasks.iter().enumerate() {
                println!("{}. {}", i + 1, task)
            }
            Ok(())
        }
        Commands::Update {
            id,
            title,
            due,
            priority,
            notes,
        } => {
            check_task_exists_by_id(&conn, id)?;
            let new_title = title.map(|n| n.join(" "));
            if let Some(ref t) = new_title {
                if t.trim().is_empty() {
                    return Err(TaskError::InvalidInput(
                        "Task title cannot be empty".to_string(),
                    ));
                }
            }
            let new_notes = notes.map(|n| n.join(" "));
            if let Some(ref due_date) = due {
                validate_date_format(&due_date)?;
            }

            update_task_by_id(&conn, id, new_title, due, priority, new_notes)?;
            let updated_task = show_task_by_id(&conn, id)?;
            println!("Update Successful! Task: {}", updated_task);
            Ok(())
        }
        Commands::Search { search_string } => {
            let search_key = search_string.join(" ");
            if search_key.trim().is_empty() {
                return Err(TaskError::InvalidInput(
                    "Search query cannot be empty".to_string(),
                ));
            }
            let matching_rows = search_by_string(&conn, search_key)?;
            if matching_rows.is_empty() {
                return Err(TaskError::NoTaskFound("No tasks found".to_string()));
            }
            for (i, matching_row) in matching_rows.iter().enumerate() {
                println!("{}. {}", i + 1, matching_row)
            }
            Ok(())
        }
        Commands::Stats {} => {
            let stats = get_stats(&conn)?;
            if stats.len() >= 6 {
                println!("Total: {}", stats[0]);
                println!("Ongoing: {}", stats[1]);
                println!("Completed: {}", stats[2]);
                println!("High priority: {}", stats[3]);
                println!("Medium priority: {}", stats[4]);
                println!("Low priority: {}", stats[5]);
            } else {
                eprintln!("Error: Unexpected stats format");
            }
            Ok(())
        }
        Commands::Help {} => {
            display_help();
            Ok(())
        }
        Commands::Clear {} => {
            clear_screen();
            Ok(())
        }
        Commands::Exit {} => {
            println!("Bye Bye...👋");
            exit_app();
        }
        Commands::Seed { reset } => {
            seed_database(conn, reset)?;
            println!("Seed complete.");
            Ok(())
        }
    }
}
