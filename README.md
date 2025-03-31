# Shortcuts App

The **Shortcuts App** is a command-line application written in Rust that allows users to manage and execute custom shortcuts. It provides an interactive CLI for adding, listing, removing, and running shortcuts.

## Features

- Add new shortcuts with a name and command.
- List all saved shortcuts.
- Remove existing shortcuts by name.
- Execute shortcuts directly from the CLI.
- Persistent storage of shortcuts in a `shortcuts.json` file.

## Commands

- `.help` - Show the help message with available commands.
- `.exit` - Exit the application.
- `.add` - Add a new shortcut.
- `.list` - List all saved shortcuts.
- `.remove` - Remove a shortcut by name.
- `<shortcut_name>` - Run a shortcut by its name.

## Installation

1. Clone the repository:
   ```bash
   git clone https://github.com/your-username/shortcuts-app.git
2. Run the project using cargo:
   ```bash
   cd Shortcuts-Rust
   cd shortcuts-app
   cargo run
