use std::fs;
use std::io;
use std::io::Write;
use rocket::response::content;
use rocket::State;
use serde::{Deserialize, Serialize};

struct TodoItem {
    id: i32,
    title: String,
    completed: bool,
}

struct TodoList {
    items: Vec<TodoItem>,
}

impl TodoList {
    fn new() -> TodoList {
        TodoList { items: Vec::new() }
    }

    fn add_item(&mut self, title: String) {
        let id = (self.items.len() + 1) as i32;
        let new_item = TodoItem {
            id,
            title,
            completed: false,
        };
        self.items.push(new_item);
    }

    fn complete_item(&mut self, id: i32) {
        for item in &mut self.items {
            if item.id == id {
                item.completed = true;
                break;
            }
        }
    }

    fn delete_item(&mut self, id: i32) {
        self.items.retain(|item| item.id != id);
    }

    fn clear_all_items(&mut self) {
        self.items.clear();
    }

    fn edit_title(&mut self, id: i32, new_title: String) {
        for item in &mut self.items {
            if item.id == id {
                item.title = new_title;
                break;
            }
        }
    }

    fn print_list(&self, show_completed: bool) -> String {
        let mut result = String::new();
        result.push_str("<ul>");
        for item in &self.items {
            if show_completed || !item.completed {
                let status = if item.completed { "Completed" } else { "Incomplete" };
                result.push_str(&format!(
                    "<li>{}. {} - {}</li>",
                    item.id, item.title, status
                ));
            }
        }
        result.push_str("</ul>");
        result
    }

    fn save_to_file(&self, filename: &str) -> io::Result<()> {
        let mut file = fs::File::create(filename)?;
        for item in &self.items {
            writeln!(file, "{}|{}|{}", item.id, item.title, item.completed)?;
        }
        Ok(())
    }

    fn load_from_file(&mut self, filename: &str) -> io::Result<()> {
        let content = fs::read_to_string(filename)?;
        for line in content.lines() {
            let parts: Vec<&str> = line.split('|').collect();
            if parts.len() == 3 {
                let id: i32 = parts[0].parse().unwrap_or_default();
                let title = parts[1].to_string();
                let completed: bool = parts[2].parse().unwrap_or(false);
                self.items.push(TodoItem { id, title, completed });
            }
        }
        Ok(())
    }
}

#[get("/")]
fn index() -> content::Html<String> {
    let html_content = fs::read_to_string("frontend/index.html").unwrap();
    content::Html(html_content)
}

#[get("/tasks")]
fn get_tasks(tasks_state: &State<TodoList>) -> content::Html<String> {
    let tasks = tasks_state.lock().unwrap();
    content::Html(tasks.print_list(false))
}

#[get("/tasks/completed")]
fn get_completed_tasks(tasks_state: &State<TodoList>) -> content::Html<String> {
    let tasks = tasks_state.lock().unwrap();
    content::Html(tasks.print_list(true))
}

// Add other routes for adding, completing, editing, and deleting tasks

fn main() {
    let mut todo_list = TodoList::new();

    // Load tasks from file if available
    if let Err(err) = todo_list.load_from_file("todo.txt") {
        println!("Error loading tasks: {}", err);
    }

    rocket::ignite()
        .mount("/", routes![index, get_tasks, get_completed_tasks])
        .manage(todo_list)
        .launch();
}
