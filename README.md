# ToDo CLI

A fast, feature-rich command-line to-do list application built in Rust.

## Features

- **Task Management** - Create, view, update, and delete tasks
- **Priority Levels** - Assign low, medium, or high priority to tasks
- **Due Dates** - Set and track due dates with overdue detection
- **Soft Delete** - Safely delete tasks with ability to restore
- **Search** - Full-text search across task titles and notes
- **Statistics** - View task completion stats at a glance
- **Persistent Storage** - SQLite database for reliable data storage

## Installation

### Prerequisites

- [Rust](https://www.rust-lang.org/tools/install) (Edition 2024)

### From crates.io (recommended)

```bash
cargo install to_do_cli
```

### From source

```bash
git clone https://github.com/Dysfunctional-Human/ToDo-List.git
cd ToDo-List
cargo build --release
```

## Usage

### Run installed binary

```bash
to_do_cli
```

### Run from source

```bash
cargo run
```

## Commands

### Adding Tasks

```bash
# Simple task
add doctor appointment

# With priority
add submit report --priority high

# With all options
add team meeting --priority medium --due 15/03/2026 --notes bring laptop
```

### Viewing Tasks

```bash
# Show specific task
show 1

# List ongoing tasks (default)
list

# List by status
list --completed
list --all

# List by priority
list --high
list --medium
list --low

# List deleted tasks
list --deleted
```

### Managing Task Status

```bash
# Mark as complete
done 1

# Reopen a completed task
reopen 1
```

### Updating Tasks

```bash
# Update any combination of fields
update 1 --title new title here
update 1 --priority low --due 20/03/2026
update 1 --notes updated notes --priority high
```

### Due Dates

```bash
# View tasks due today
due --today

# View tasks due tomorrow
due --tomorrow
```

### Deleting Tasks

```bash
# Soft delete (can be restored)
delete 1

# Restore a deleted task
restore 1

# Permanently delete
purge 1

# Permanently delete all deleted tasks
purge --all
```

### Search & Stats

```bash
# Search tasks
search meeting

# View statistics
stats
```

### Utility

```bash
# Show help
help

# Clear screen
clear

# Exit application
exit
```

## Date Format

All dates must be in `dd/mm/yyyy` format:

- `15/03/2026` - Valid
- `2026-03-15` - Invalid
- `03/15/2026` - Invalid

## Tech Stack

- **Language:** Rust (Edition 2024)
- **Database:** SQLite via [rusqlite](https://crates.io/crates/rusqlite)
- **CLI Framework:** [Clap](https://crates.io/crates/clap) v4.5
- **Date/Time:** [Chrono](https://crates.io/crates/chrono)

## Project Structure

```bash
ToDo-List/
├── src/
│   ├── main.rs
│   ├── lib.rs
│   ├── cli.rs
│   ├── models.rs
│   ├── db.rs
│   └── mod.rs
├── Cargo.toml
└── README.md
```

## Roadmap

- [ ] Unit tests for input validation
- [ ] Notes editor with multi-line support
- [ ] Multi-device sync with authentication
- [ ] WhatsApp integration for mobile access
- [ ] Make sure all doc and clippy checks pass

## License

LGPL-2.1
