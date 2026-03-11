// Binary to seed the database with mock data
// Run with: cargo run --bin seed

use rusqlite::Connection;

// Import the seed module
mod seed_data {
    include!("../seed.rs");
}

fn main() -> rusqlite::Result<()> {
    println!("Opening database connection...");
    let conn = Connection::open("todo.db")?;

    // Create table if it doesn't exist
    println!("Initializing database schema...");
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

    println!("Seeding database with mock data...");
    seed_data::seed_database(&conn, false)?;

    println!("Done! Use 'cargo run' to start the todo app and view the tasks.");
    Ok(())
}
