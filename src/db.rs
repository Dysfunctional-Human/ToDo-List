use crate::models::{PriorityOrder, Task, TaskError, TaskStatus};
use chrono::{Duration, Local, NaiveDate};
use rusqlite::{Connection, Row, params};
use std::process::{self, Command};

pub fn init_db() -> rusqlite::Result<Connection> {
    let conn = Connection::open("todo.db")?;

    conn.execute_batch(
        "
        CREATE TABLE IF NOT EXISTS tasks (
            id              INTEGER PRIMARY KEY AUTOINCREMENT,
            title           TEXT NOT NULL,
            status          TEXT NOT NULL,
            created_at      TEXT NOT NULL,
            updated_at      TEXT,
            completed_at    TEXT,
            deleted_at      TEXT,
            due_at          TEXT,
            priority        TEXT,
            notes           TEXT
        );
        ",
    )?;
    Ok(conn)
}

pub fn validate_date_format(date_str: &str) -> Result<(), TaskError> {
    NaiveDate::parse_from_str(date_str, "%d/%m/%Y").map_err(|_| {
        TaskError::InvalidDateFormat(format!(
            "Invalid date '{}'. Use dd/mm/yyyy format",
            date_str
        ))
    })?;
    Ok(())
}

pub fn create_task(conn: &Connection, new_task: &Task) -> Result<u64, TaskError> {
    let priority = new_task.priority.as_ref().map(|p| p.as_str());
    let now = Local::now().format("%d/%m/%Y").to_string();

    conn.execute(
        "INSERT INTO tasks (title, status, created_at, due_at,
                            priority, notes)
         VALUES (?1, ?2, ?3, ?4, ?5, ?6)
        ",
        params![
            &new_task.title,
            &new_task.status.as_str(),
            now,
            &new_task.due_at,
            priority,
            &new_task.notes
        ],
    )?;

    let new_id = conn.last_insert_rowid();

    Ok(new_id as u64)
}

fn parse_all_columns(row: &Row<'_>) -> rusqlite::Result<Task> {
    let task_status: String = row.get(2)?;
    let status = match task_status.as_str() {
        "Ongoing" => TaskStatus::Ongoing,
        "Completed" => TaskStatus::Completed,
        _ => {
            eprintln!(
                "Warning: Unknown status '{}' in database, treating as Ongoing",
                task_status
            );
            TaskStatus::Ongoing
        }
    };

    let task_priority: Option<String> = row.get(8)?;
    let priority = task_priority.and_then(|p| match p.as_str() {
        "High" => Some(PriorityOrder::High),
        "Medium" => Some(PriorityOrder::Medium),
        "Low" => Some(PriorityOrder::Low),
        unknown => {
            eprintln!("Warning: Unknown priority '{}' in database", unknown);
            None
        }
    });

    return Ok(Task {
        id: row.get(0)?,
        title: row.get(1)?,
        status: status,
        created_at: row.get(3)?,
        updated_at: row.get(4)?,
        completed_at: row.get(5)?,
        deleted_at: row.get(6)?,
        due_at: row.get(7)?,
        priority: priority,
        notes: row.get(9)?,
    });
}

pub fn show_task_by_id(conn: &Connection, task_id: u64) -> Result<Task, TaskError> {
    let task_info = conn.query_row(
        "SELECT * FROM tasks WHERE id = ?1",
        params![task_id],
        |row| parse_all_columns(row),
    )?;

    Ok(task_info)
}

pub fn check_for_redundancy(conn: &Connection, new_task: &Task) -> Result<(), TaskError> {
    match conn.query_row(
        "SELECT id FROM tasks WHERE (title = ?1 OR (notes IS NOT NULL AND notes = ?2)) AND deleted_at IS NULL",
        params![new_task.title, new_task.notes],
        |row| Ok(row.get::<_, u64>(0)?)
    ) {
        Ok(_) => return Err(TaskError::DuplicateTask("Task can't have duplicate title and/or notes".to_string())),
        Err(rusqlite::Error::QueryReturnedNoRows) => Ok(()),
        Err(e) => Err(TaskError::DatabaseError(e))
    }
}

pub fn check_task_exists_by_id(conn: &Connection, id: u64) -> Result<(), TaskError> {
    match conn.query_row("SELECT id FROM tasks WHERE id = ?1", params![id], |row| {
        row.get::<_, u64>(0)
    }) {
        Ok(_) => Ok(()),
        Err(rusqlite::Error::QueryReturnedNoRows) => Err(TaskError::NoTaskFound(
            "Task corresponding to this id does not exist".to_string(),
        )),
        Err(e) => return Err(TaskError::DatabaseError(e)),
    }
}

pub fn get_tasks_by_status(
    conn: &Connection,
    all: bool,
    completed: bool,
    ongoing: bool,
) -> Result<Vec<Task>, TaskError> {
    let status_str = if completed {
        "Completed"
    } else if ongoing {
        "Ongoing"
    } else {
        return Err(TaskError::InvalidInput(
            "Must specify either completed or ongoing".to_string(),
        ));
    };
    let mut query = if all {
        conn.prepare("SELECT * FROM tasks WHERE status = ?1")?
    } else {
        conn.prepare("SELECT * FROM tasks WHERE status = ?1 AND deleted_at IS NULL")?
    };

    let rows = query.query_map([status_str], |row| parse_all_columns(row))?;
    let tasks: Vec<Task> = rows.collect::<Result<Vec<_>, _>>()?;
    Ok(tasks)
}

pub fn get_tasks_by_priority(
    conn: &Connection,
    low: bool,
    medium: bool,
    high: bool,
    all: bool,
) -> Result<Vec<Task>, TaskError> {
    let priority_str = if low {
        "Low"
    } else if medium {
        "Medium"
    } else if high {
        "High"
    } else {
        return Err(TaskError::InvalidInput(
            "Must specify either low, medium or high".to_string(),
        ));
    };

    let mut query = if all {
        conn.prepare("SELECT * FROM tasks WHERE priority = ?1")?
    } else {
        conn.prepare("SELECT * FROM tasks WHERE priority = ?1 AND deleted_at IS NULL")?
    };
    let rows = query.query_map([priority_str], |row| parse_all_columns(row))?;
    let tasks: Vec<Task> = rows.collect::<Result<Vec<_>, _>>()?;
    Ok(tasks)
}

pub fn get_all_tasks(conn: &Connection) -> Result<Vec<Task>, TaskError> {
    let mut query = conn.prepare("SELECT * FROM tasks WHERE deleted_at IS NULL")?;
    let rows = query.query_map([], |row| parse_all_columns(row))?;
    let tasks: Vec<Task> = rows.collect::<Result<Vec<_>, _>>()?;
    Ok(tasks)
}

pub fn get_deleted_tasks(conn: &Connection, all: bool) -> Result<Vec<Task>, TaskError> {
    let mut query = if all {
        conn.prepare("SELECT * FROM tasks")?
    } else {
        conn.prepare("SELECT * FROM tasks WHERE deleted_at IS NOT NULL")?
    };
    let rows = query.query_map([], |row| parse_all_columns(row))?;
    let tasks: Vec<Task> = rows.collect::<Result<Vec<_>, _>>()?;
    Ok(tasks)
}

pub fn update_status(
    conn: &Connection,
    id: u64,
    updated_status: TaskStatus,
) -> Result<(), TaskError> {
    let date = Local::now().format("%d/%m/%Y").to_string();
    if updated_status == TaskStatus::Completed {
        let rows_affected = conn.execute(
            "UPDATE tasks SET status = ?1, updated_at = ?3, completed_at = ?3 WHERE id = ?2 AND deleted_at IS NULL", 
            params![updated_status.as_str(), id, date]
        )?;
        if rows_affected == 0 {
            return Err(TaskError::NoTaskFound(
                "No task found for the given id, make sure the task isn't deleted".to_string(),
            ));
        }
    } else if updated_status == TaskStatus::Ongoing {
        let rows_affected = conn.execute(
            "UPDATE tasks SET status = ?1, updated_at = ?3, completed_at = NULL WHERE id = ?2 AND deleted_at IS NULL", 
            params![updated_status.as_str(), id, date]
        )?;
        if rows_affected == 0 {
            return Err(TaskError::NoTaskFound(
                "No task found for the given id, make sure the task isn't deleted".to_string(),
            ));
        }
    } else {
        return Err(TaskError::InvalidInput("Invalid Task Status".to_string()));
    }
    Ok(())
}

pub fn soft_delete_task(conn: &Connection, id: u64) -> Result<(), TaskError> {
    let rows_affected = conn.execute(
        "UPDATE tasks SET deleted_at = ?1, updated_at = ?1 WHERE id = ?2 AND deleted_at IS NULL",
        params![Local::now().format("%d/%m/%Y").to_string(), id],
    )?;
    if rows_affected == 0 {
        return Err(TaskError::InvalidInput(
            "Task is already deleted".to_string(),
        ));
    }
    Ok(())
}

pub fn restore_task(conn: &Connection, id: u64) -> Result<(), TaskError> {
    let rows_affected = conn.execute(
        "UPDATE tasks SET deleted_at = NULL, updated_at = ?2 WHERE id = ?1 AND deleted_at IS NOT NULL",
        params![id, Local::now().format("%d/%m/%Y").to_string()]
    )?;
    if rows_affected == 0 {
        return Err(TaskError::InvalidInput("Task is not deleted".to_string()));
    }
    Ok(())
}

pub fn purge_task(conn: &Connection, id: Option<u64>, all: bool) -> Result<(), TaskError> {
    if let Some(task_id) = id {
        check_task_exists_by_id(conn, task_id)?;
        let rows_affected = conn.execute(
            "DELETE FROM tasks WHERE id = ?1 AND deleted_at IS NOT NULL",
            params![task_id],
        )?;
        if rows_affected == 0 {
            return Err(TaskError::InvalidInput(
                "Task is not deleted. Use 'delete' first".to_string(),
            ));
        }
    } else if all {
        let rows_affected =
            conn.execute("DELETE FROM tasks WHERE deleted_at IS NOT NULL", params![])?;
        if rows_affected == 0 {
            return Err(TaskError::InvalidInput("No tasks to purge".to_string()));
        }
    } else {
        return Err(TaskError::InvalidInput(
            "Must specify either --all or a valid id".to_string(),
        ));
    }

    Ok(())
}

pub fn get_due_tasks(
    conn: &Connection,
    today: bool,
    tomorrow: bool,
) -> Result<Vec<Task>, TaskError> {
    let today_str = Local::now().format("%d/%m/%Y").to_string();
    let tomorrow_str = (Local::now() + Duration::days(1))
        .format("%d/%m/%Y")
        .to_string();

    let (query_str, params): (&str, Vec<&str>) = if today && tomorrow {
        (
            "SELECT * FROM tasks WHERE (due_at = ?1 OR due_at = ?2) AND deleted_at IS NULL AND status = 'Ongoing'",
            vec![&today_str, &tomorrow_str],
        )
    } else if today {
        (
            "SELECT * FROM tasks WHERE due_at = ?1 AND deleted_at IS NULL AND status = 'Ongoing'",
            vec![&today_str],
        )
    } else if tomorrow {
        (
            "SELECT * FROM tasks WHERE due_at = ?1 AND deleted_at IS NULL AND status = 'Ongoing'",
            vec![&tomorrow_str],
        )
    } else {
        return Err(TaskError::InvalidInput(
            "Must specify either --today or --tomorrow".to_string(),
        ));
    };

    let mut query = conn.prepare(query_str)?;
    let rows = query.query_map(rusqlite::params_from_iter(params), |row| {
        parse_all_columns(row)
    })?;
    let tasks: Vec<Task> = rows.collect::<Result<Vec<_>, _>>()?;
    Ok(tasks)
}

pub fn update_task_by_id(
    conn: &Connection,
    id: u64,
    title: Option<String>,
    due: Option<String>,
    priority: Option<PriorityOrder>,
    notes: Option<String>,
) -> Result<(), TaskError> {
    let date = Local::now().format("%d/%m/%Y").to_string();
    let mut set_clauses: Vec<String> = vec![];
    let mut params: Vec<Box<dyn rusqlite::ToSql>> = vec![];

    if let Some(t) = title {
        set_clauses.push("title = ?".to_string());
        params.push(Box::new(t));
    }

    if let Some(d) = due {
        set_clauses.push("due_at = ?".to_string());
        params.push(Box::new(d));
    }

    if let Some(p) = priority {
        set_clauses.push("priority = ?".to_string());
        params.push(Box::new(p.as_str().to_string()));
    }

    if let Some(n) = notes {
        set_clauses.push("notes = ?".to_string());
        params.push(Box::new(n));
    }

    set_clauses.push("updated_at = ?".to_string());
    params.push(Box::new(date));

    if set_clauses.len() == 1 {
        return Err(TaskError::InvalidInput("Nothing to update".to_string()));
    }

    params.push(Box::new(id));

    let sql = format!(
        "UPDATE tasks SET {} WHERE id = ? AND deleted_at IS NULL",
        set_clauses.join(", ")
    );
    let rows_affected = conn.execute(&sql, rusqlite::params_from_iter(params))?;
    if rows_affected == 0 {
        return Err(TaskError::InvalidInput(
            "Please make sure a task exists for the given id, and it is not deleted".to_string(),
        ));
    }
    Ok(())
}

pub fn search_by_string(conn: &Connection, search_key: String) -> Result<Vec<Task>, TaskError> {
    // Escape SQL LIKE wildcards
    let escaped = search_key
        .replace("\\", "\\\\") // Escape backslash first
        .replace("%", "\\%")
        .replace("_", "\\_");
    let pattern = format!("%{}%", escaped);

    let mut query = conn.prepare("SELECT * FROM tasks WHERE (title LIKE ?1 ESCAPE '\\' OR notes LIKE ?1 ESCAPE '\\') AND deleted_at IS NULL")?;
    let rows = query.query_map([pattern], |row| parse_all_columns(row))?;
    let tasks: Vec<Task> = rows.collect::<Result<Vec<_>, _>>()?;
    Ok(tasks)
}

pub fn get_stats(conn: &Connection) -> Result<Vec<u64>, TaskError> {
    let stats = conn.query_row(
        "
        SELECT 
            COALESCE(SUM(CASE WHEN deleted_at IS NULL THEN 1 ELSE 0 END), 0) as total,
            COALESCE(SUM(CASE WHEN status = 'Ongoing' AND deleted_at IS NULL THEN 1 ELSE 0 END), 0) as ongoing,
            COALESCE(SUM(CASE WHEN status = 'Completed' AND deleted_at IS NULL THEN 1 ELSE 0 END), 0) as completed,
            COALESCE(SUM(CASE WHEN priority = 'High' AND deleted_at IS NULL THEN 1 ELSE 0 END), 0) as high_priority,
            COALESCE(SUM(CASE WHEN priority = 'Medium' AND deleted_at IS NULL THEN 1 ELSE 0 END), 0) as medium_priority,
            COALESCE(SUM(CASE WHEN priority = 'Low' AND deleted_at IS NULL THEN 1 ELSE 0 END), 0) as low_priority
        FROM tasks
        ",
        [],
        |row| Ok(vec![
            row.get(0)?,
            row.get(1)?,
            row.get(2)?,
            row.get(3)?,
            row.get(4)?,
            row.get(5)?
        ])
    )?;
    Ok(stats)
}

pub fn clear_screen() {
    #[cfg(target_os = "windows")]
    {
        let _ = Command::new("cmd").args(["/C", "cls"]).status();
    }
    #[cfg(not(target_os = "windows"))]
    {
        let _ = Command::new("clear").status();
    }
}

pub fn exit_app() -> ! {
    process::exit(0);
}
