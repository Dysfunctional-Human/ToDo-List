#![allow(dead_code)]
// #![allow(unused_imports)]
use rusqlite::{Connection, Result, params};
use crate::models::{Task, TaskError, TaskStatus, PriorityOrder};
use chrono::Utc;

pub fn init_db() -> Result<Connection> {
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

pub fn create_task(
    conn: &Connection,
    new_task: &Task
) -> Result<u64, TaskError> {
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

    let new_id = conn.last_insert_rowid();

    Ok(new_id as u64)
}

pub fn show_task_by_id(
    conn: &Connection,
    task_id: u64
) -> Result<Task, TaskError> {
    let task_info = conn.query_row(
        "SELECT * FROM tasks WHERE id = ?1",
        params![task_id],
        |row| {
            let task_status: String = row.get(2)?;
            let status = match task_status.as_str() {
                    "Ongoing" => TaskStatus::Ongoing,
                    "Completed" => TaskStatus::Completed,
                    _ => return Err(rusqlite::Error::InvalidQuery)
            };

            let task_priority: Option<String> = row.get(8)?;
            let priority = task_priority.and_then(|p| {
                match p.as_str() {
                    "High" => Some(PriorityOrder::High),
                    "Medium" => Some(PriorityOrder::Medium),
                    "Low" => Some(PriorityOrder::Low),
                    _ => None
                }
            });

            Ok(Task {
                id: row.get(0)?,
                title: row.get(1)?,
                status: status,
                created_at: row.get(3)?,
                updated_at: row.get(4)?,
                completed_at: row.get(5)?,
                deleted_at: row.get(6)?,
                due_at: row.get(7)?,
                priority: priority,
                notes: row.get(9)?
            })
        }
    )?;

    Ok(task_info)
}

pub fn check_for_redundancy(
    conn: &Connection,
    new_task: &Task
) -> Result<(), TaskError> {
    match conn.query_row(
        "SELECT id FROM tasks WHERE title = ?1 OR (notes = ?2 AND notes IS NOT NULL)",
        params![new_task.title, new_task.notes],
        |row| Ok(row.get::<_, i64>(0)?)
    ) {
        Ok(_) => return Err(TaskError::DuplicateTask("Task can't have duplicate title and/or notes".to_string())),
        Err(rusqlite::Error::QueryReturnedNoRows) => Ok(()),
        Err(_) => Ok(())
    }
}