use chrono::{Duration, Local};
use rusqlite::{Connection, params};

/// Seeds the database with diverse mock data for testing
pub fn seed_database(conn: &Connection, reset: bool) -> rusqlite::Result<()> {
    if reset {
        conn.execute("DELETE FROM tasks", [])?;
    }
    let today = Local::now();
    let yesterday = today - Duration::days(1);
    let last_week = today - Duration::days(7);
    let two_weeks_ago = today - Duration::days(14);
    let tomorrow = today + Duration::days(1);
    let next_week = today + Duration::days(7);

    // Helper to format dates
    let fmt = |date: chrono::DateTime<Local>| date.format("%d/%m/%Y").to_string();

    // Clear existing data (optional - comment out if you want to preserve existing tasks)
    // conn.execute("DELETE FROM tasks", [])?;

    // 1. High priority ongoing tasks with various due dates
    conn.execute(
        "INSERT INTO tasks (title, status, created_at, due_at, priority, notes)
         VALUES (?1, ?2, ?3, ?4, ?5, ?6)",
        params![
            "Complete tax returns",
            "Ongoing",
            fmt(two_weeks_ago),
            fmt(tomorrow),
            "High",
            "Need to gather all receipts and W2 forms"
        ],
    )?;

    conn.execute(
        "INSERT INTO tasks (title, status, created_at, due_at, priority, notes)
         VALUES (?1, ?2, ?3, ?4, ?5, ?6)",
        params![
            "Prepare presentation for client meeting",
            "Ongoing",
            fmt(yesterday),
            fmt(today),
            "High",
            "Include Q4 metrics and growth projections"
        ],
    )?;

    conn.execute(
        "INSERT INTO tasks (title, status, created_at, due_at, priority)
         VALUES (?1, ?2, ?3, ?4, ?5)",
        params![
            "Review security audit report",
            "Ongoing",
            fmt(last_week),
            fmt(next_week),
            "High"
        ],
    )?;

    // 2. Medium priority ongoing tasks
    conn.execute(
        "INSERT INTO tasks (title, status, created_at, priority, notes)
         VALUES (?1, ?2, ?3, ?4, ?5)",
        params![
            "Update project documentation",
            "Ongoing",
            fmt(last_week),
            "Medium",
            "Add API endpoints and examples"
        ],
    )?;

    conn.execute(
        "INSERT INTO tasks (title, status, created_at, due_at, priority)
         VALUES (?1, ?2, ?3, ?4, ?5)",
        params![
            "Schedule team building event",
            "Ongoing",
            fmt(yesterday),
            fmt(today + Duration::days(30)),
            "Medium"
        ],
    )?;

    // 3. Low priority ongoing tasks
    conn.execute(
        "INSERT INTO tasks (title, status, created_at, priority, notes)
         VALUES (?1, ?2, ?3, ?4, ?5)",
        params![
            "Organize desk workspace",
            "Ongoing",
            fmt(last_week),
            "Low",
            "File papers and clean keyboard"
        ],
    )?;

    conn.execute(
        "INSERT INTO tasks (title, status, created_at, due_at, priority)
         VALUES (?1, ?2, ?3, ?4, ?5)",
        params![
            "Read industry newsletter",
            "Ongoing",
            fmt(yesterday),
            fmt(next_week),
            "Low"
        ],
    )?;

    // 4. Tasks without priority
    conn.execute(
        "INSERT INTO tasks (title, status, created_at, notes)
         VALUES (?1, ?2, ?3, ?4)",
        params![
            "Buy groceries",
            "Ongoing",
            fmt(today),
            "Milk, eggs, bread, coffee"
        ],
    )?;

    conn.execute(
        "INSERT INTO tasks (title, status, created_at, due_at)
         VALUES (?1, ?2, ?3, ?4)",
        params![
            "Call dentist for appointment",
            "Ongoing",
            fmt(yesterday),
            fmt(today + Duration::days(5))
        ],
    )?;

    // 5. Completed tasks with various priorities
    conn.execute(
        "INSERT INTO tasks (title, status, created_at, completed_at, priority, notes)
         VALUES (?1, ?2, ?3, ?4, ?5, ?6)",
        params![
            "Submit quarterly report",
            "Completed",
            fmt(two_weeks_ago),
            fmt(last_week),
            "High",
            "Submitted to finance department"
        ],
    )?;

    conn.execute(
        "INSERT INTO tasks (title, status, created_at, completed_at, due_at, priority)
         VALUES (?1, ?2, ?3, ?4, ?5, ?6)",
        params![
            "Fix login bug in production",
            "Completed",
            fmt(last_week),
            fmt(last_week + Duration::days(1)),
            fmt(last_week + Duration::days(2)),
            "High"
        ],
    )?;

    conn.execute(
        "INSERT INTO tasks (title, status, created_at, completed_at, priority, notes)
         VALUES (?1, ?2, ?3, ?4, ?5, ?6)",
        params![
            "Update team wiki",
            "Completed",
            fmt(two_weeks_ago),
            fmt(yesterday),
            "Medium",
            "Added onboarding guide and troubleshooting section"
        ],
    )?;

    conn.execute(
        "INSERT INTO tasks (title, status, created_at, completed_at, priority)
         VALUES (?1, ?2, ?3, ?4, ?5)",
        params![
            "Water office plants",
            "Completed",
            fmt(yesterday),
            fmt(today),
            "Low"
        ],
    )?;

    // 6. Deleted tasks (with deleted_at timestamp)
    conn.execute(
        "INSERT INTO tasks (title, status, created_at, deleted_at, priority, notes)
         VALUES (?1, ?2, ?3, ?4, ?5, ?6)",
        params![
            "Attend conference in Seattle",
            "Ongoing",
            fmt(two_weeks_ago),
            fmt(yesterday),
            "Medium",
            "Event was cancelled due to weather"
        ],
    )?;

    conn.execute(
        "INSERT INTO tasks (title, status, created_at, deleted_at, priority)
         VALUES (?1, ?2, ?3, ?4, ?5)",
        params![
            "Review old project proposal",
            "Ongoing",
            fmt(last_week),
            fmt(yesterday),
            "Low"
        ],
    )?;

    conn.execute(
        "INSERT INTO tasks (title, status, created_at, completed_at, deleted_at, priority)
         VALUES (?1, ?2, ?3, ?4, ?5, ?6)",
        params![
            "Duplicate task to be removed",
            "Completed",
            fmt(two_weeks_ago),
            fmt(last_week),
            fmt(yesterday),
            "High"
        ],
    )?;

    // 7. Edge cases - tasks with very long titles/notes
    conn.execute(
        "INSERT INTO tasks (title, status, created_at, priority, notes)
         VALUES (?1, ?2, ?3, ?4, ?5)",
        params![
            "Research and evaluate different cloud infrastructure providers for upcoming migration project",
            "Ongoing",
            fmt(today),
            "Medium",
            "Compare AWS, GCP, and Azure pricing models. Consider scalability, security features, compliance certifications, and integration with existing tools. Document findings in shared drive."
        ]
    )?;

    // 8. Tasks with special characters
    conn.execute(
        "INSERT INTO tasks (title, status, created_at, notes)
         VALUES (?1, ?2, ?3, ?4)",
        params![
            "Fix bug in user's profile page",
            "Ongoing",
            fmt(today),
            "Error message: 'Cannot read property \"name\" of undefined'"
        ],
    )?;

    println!("Successfully seeded database with {} mock tasks!", 18);
    Ok(())
}
