# Rusty Manager

This is a simple task manager application written in Rust. It allows you to add, modify, and delete tasks from a list.

## Controls

- `Tab`: Switch to input mode to add a new task.
- `Enter`: In task mode, toggle the completion status of the selected task. In input mode, add the entered task to the list. In edit mode confirm name change.
- `Up Key`/`Down Key`: Navigate through the list of tasks.
- `M`: Modify the selected task.
- `X`: Delete the selected task (a confirmation prompt will appear).
  - `Y`/`N`: Confirm or cancel the deletion of a task.
- `Q`: Quit the application.

## Installation

To install and run this application, you need to have Rust and Cargo installed on your machine. If you haven't installed them yet, you can do so by following the instructions on the [official Rust website](https://www.rust-lang.org/tools/install).

Once you have Rust and Cargo installed:

1. Clone this repository.
2. Navigate to the project's directory.
3. Run `cargo build --release` to build the application.
4. Run `./target/release/your-app-name` to start the application.

## License

This project is licensed under the Apache License 2.0 - see the [LICENSE](LICENSE) file for details.
