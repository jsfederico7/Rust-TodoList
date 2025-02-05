🌟 Rust Todo List Application 🌟

A simple Todo List program written in Rust. This project allows users to manage their tasks by adding, listing, and removing them, with automatic persistence through a JSON file (todos.json).

🚀 Features
📋 Add tasks: define a description and generate a unique identifier.
📜 List tasks: display the status (✓ Completed or ✗ Incomplete) along with the creation date.
❌ Remove tasks: delete a specific task by selecting its identifier.
💾 Data persistence: tasks are saved to a JSON file (todos.json).
🌈 Colored terminal interface: better user experience using the colored crate.

🔧 Technologies Used

Rust: main language.
Crates:
chrono: for date and time management.
colored: for colored output in the terminal.
serde and serde_json: for JSON serialization and deserialization.

📦 Installation
Clone the repository:

git clone https://github.com/jsfederico7/Rust-TodoLis.git
cd rust-todo-list
Install the dependencies:

cargo build
Run the program:


cargo run
📝 Usage
Upon running the program, select one of the following options:

🌟 Welcome to Rust Todo! 🌟

Menu:
1. Add Todo
2. List Todos
3. Remove Todo
4. Quit

Choose an option:

Follow the instructions based on your choice.
Tasks are automatically stored in todos.json.

🚧 Example Execution

🌟 Welcome to Rust Todo! 🌟

Menu:
1. Add Todo
2. List Todos
3. Remove Todo
4. Quit
Choose an option: 1

Enter todo description:

Learn Rust
✔ Todo added!
⚙ Tests

This project includes unit tests (cargo test) for:

Adding a task.
Removing a task.
Saving and loading tasks from a file.

Run the tests with:
cargo test
