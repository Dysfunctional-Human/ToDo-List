use rusqlite::Connection;
use std::io::Write;
use std::io::stdin; // To read user input
use std::io::stdout; // To display output
use to_do_cli::db::init_db; // Use the library crate's db module

fn runprompt(conn: &Connection) {
    loop {
        let mut output = stdout(); // Creates a handle standard output. Made mutable so it 
        // can be flushed

        print!("(ToDo list) > "); // Using print!() macro instead of print;n!() since the 
        // latter auto-flushes, we don't want that.
        output
            .flush() // Forces prompt to appear immediately
            .expect("can't flush the stdout"); // Crash with this message if fails

        let mut buffer = String::new(); // Empty string to hold user input
        stdin()
            .read_line(&mut buffer) // Reads input until enter is pressed, and stores in buffer
            .expect("Cannot readline");

        let args: Vec<&str> = buffer
            .split_whitespace() // splits words about whitespaces
            .collect(); // converts them into &str (read-only string)
        // literals and stores them in args vector
        // for arg in args {
        //     println!("{}", arg)
        // }

        if args.is_empty() {
            continue;
        }

        match to_do_cli::parse_arguments(&conn, args) {
            Ok(()) => {}
            Err(e) => eprintln!("Error: {:?}", e),
        }
    }
}

fn main() {
    let conn = init_db().expect("Failed to initialize the database");
    runprompt(&conn); // Passes to runprompt to fill the vector
}
