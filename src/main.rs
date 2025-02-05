use colored::*;
use serde::{Deserialize, Serialize};
use std::fs;
use std::io::{self, Write};
use chrono::{DateTime, Utc};

#[derive(Debug, Serialize, Deserialize)]
struct Todo {
    id: usize,
    description: String,
    completed: bool,
    created_at: DateTime<Utc>,
}

impl Todo {
    fn new(id: usize, description: String) -> Self {
        Todo {
            id,
            description,
            completed: false,
            created_at: Utc::now(),
        }
    }
}

fn main() {
    println!("{}", "🌟 Welcome to Rust Todo! 🌟".bright_cyan().bold());

    let mut todos = load_todos().unwrap_or_else(|e| {
        eprintln!("{} {}", "Warning:".yellow(), e);
        Vec::new()
    });

    loop {
        print_menu();

        let choice = read_line().trim().to_lowercase();
        match choice.as_str() {
            "1" | "add" => add_todo(&mut todos),
            "2" | "list" => list_todos(&todos),
            "3" | "remove" => remove_todo(&mut todos),
            "4" | "quit" | "exit" => break,
            _ => println!("{}", "Invalid choice, try again.".red()),
        }
    }

    if let Err(e) = save_todos(&todos) {
        eprintln!("{} {}", "Error saving todos:".red(), e);
    }
}

fn print_menu() {
    println!("\n{}", "Menu:".bright_blue().underline());
    println!("1. {} Todo", "Add".bright_green());
    println!("2. {} Todos", "List".bright_yellow());
    println!("3. {} Todo", "Remove".bright_red());
    println!("4. {}", "Quit".bright_magenta());
    print!("{} ", "Choose an option:".bright_cyan());
    io::stdout().flush().unwrap();
}

fn read_line() -> String {
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    input.trim().to_string()
}

fn load_todos() -> io::Result<Vec<Todo>> {
    let data = fs::read_to_string("todos.json")?;
    Ok(serde_json::from_str(&data)?)
}

fn save_todos(todos: &[Todo]) -> io::Result<()> {
    let json = serde_json::to_string_pretty(todos)?;
    fs::write("todos.json", json)
}

fn add_todo(todos: &mut Vec<Todo>) {
    println!("\n{}", "Enter todo description:".bright_cyan());
    let description = read_line();

    if description.is_empty() {
        println!("{}", "Description cannot be empty!".red());
        return;
    }

    let new_id = todos.len() + 1;
    todos.push(Todo::new(new_id, description));
    println!("{} {}", "✔".bright_green(), "Todo added!".bright_green());
}

fn list_todos(todos: &[Todo]) {
    println!("\n{}", "Your Todos:".bright_yellow().underline());

    if todos.is_empty() {
        println!("{}", "No todos yet! Add your first one! 🎉".bright_blue());
        return;
    }

    for todo in todos {
        let status = if todo.completed {
            "✓".bright_green()
        } else {
            "✗".bright_red()
        };

        println!(
            "{} {} [{}] {}",
            format!("#{}", todo.id).bright_cyan(),
            status,
            todo.created_at.format("%Y-%m-%d %H:%M").to_string().dimmed(),
            todo.description
        );
    }
}

fn remove_todo(todos: &mut Vec<Todo>) {
    if todos.is_empty() {
        println!("{}", "No todos to remove!".bright_blue());
        return;
    }

    list_todos(todos);
    println!("\n{}", "Enter the ID of the todo to remove:".bright_cyan());

    match read_line().parse::<usize>() {
        Ok(id) => {
            if let Some(index) = todos.iter().position(|t| t.id == id) {
                todos.remove(index);
                println!("{} {}", "✔".bright_green(), "Todo removed!".bright_green());
            } else {
                println!("{}", "❌ No todo found with that ID".red());
            }
        }
        Err(_) => println!("{}", "❌ Please enter a valid number".red()),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add_todo() {
        let mut todos = vec![];
        add_todo(&mut todos);
        assert_eq!(todos.len(), 1);
    }

    #[test]
    fn test_remove_todo() {
        let mut todos = vec![Todo::new(1, "Test todo".into())];
        remove_todo(&mut todos);
        assert!(todos.is_empty());
    }

    #[test]
    fn test_save_load() {
        let todos = vec![Todo::new(1, "Test todo".into())];
        save_todos(&todos).unwrap();
        let loaded = load_todos().unwrap();
        assert_eq!(todos.len(), loaded.len());
    }
}