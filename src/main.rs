use std::fs::{File, OpenOptions};
use std::io::{self, BufRead, BufReader, Write};

// ===== Struct and Enum =====

#[derive(Debug, Clone)]
struct Task {
    description: String,
    status: TaskStatus,
    date: String,
}

#[derive(Debug, Clone)]
enum TaskStatus {
    Pending,
    Completed,
}

impl TaskStatus {
    fn from_str(s: &str) -> TaskStatus {
        match s {
            "Completed" => TaskStatus::Completed,
            _ => TaskStatus::Pending,
        }
    }

    fn as_str(&self) -> &str {
        match self {
            TaskStatus::Pending => "Pending",
            TaskStatus::Completed => "Completed",
        }
    }
}

// ===== File I/O =====

fn load_tasks(filename: &str) -> io::Result<Vec<Task>> {
    let file = File::open(filename);

    let mut tasks = Vec::<Task>::new();

    if let Ok(f) = file {
        for line in BufReader::new(f).lines() {
            if let Ok(content) = line {
                let parts: Vec<&str> = content.split('|').collect();
                if parts.len() == 3 {
                    let status = TaskStatus::from_str(parts[0]);
                    let description = parts[1].to_string();
                    let date = parts[2].to_string();

                    tasks.push(Task {
                        description,
                        status,
                        date,
                    });
                }
            }
        }
    }

    Ok(tasks)
}

fn save_tasks(filename: &str, tasks: &Vec<Task>) -> io::Result<()> {
    let mut file = File::create(filename)?;

    for task in tasks {
        let line = format!(
            "{}|{}|{}\n",
            task.status.as_str(),
            task.description,
            task.date
        );
        file.write_all(line.as_bytes())?;
    }

    Ok(())
}

// ===== User Actions =====

fn add_task(tasks: &mut Vec<Task>) {
    let mut description = String::new();
    let date = today_string();

    print!("Enter task description: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut description).unwrap();

    let task = Task {
        description: description.trim().to_string(),
        status: TaskStatus::Pending,
        date,
    };

    tasks.push(task);

    println!("Task added!");
}

fn list_tasks(tasks: &Vec<Task>, show: &str) {
    println!("\n=== {} TASKS ===", show);

    for (i, task) in tasks.iter().enumerate() {
        let show_it = match show {
            "Pending" => matches!(task.status, TaskStatus::Pending),
            "Completed" => matches!(task.status, TaskStatus::Completed),
            "All" => true,
            _ => false,
        };

        if show_it {
            println!(
                "{}. [{}] {} ({})",
                i + 1,
                task.status.as_str(),
                task.description,
                task.date
            );
        }
    }
}

fn mark_completed(tasks: &mut Vec<Task>) {
    let mut input = String::new();
    list_tasks(tasks, "Pending");

    print!("\nEnter task # to mark completed: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut input).unwrap();

    if let Ok(index) = input.trim().parse::<usize>() {
        if index >= 1 && index <= tasks.len() {
            tasks[index - 1].status = TaskStatus::Completed;
            println!("Task marked completed!");
            return;
        }
    }

    println!("Invalid task number.");
}

fn delete_task(tasks: &mut Vec<Task>) {
    let mut input = String::new();
    list_tasks(tasks, "All");

    print!("\nEnter task # to delete: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut input).unwrap();

    if let Ok(index) = input.trim().parse::<usize>() {
        if index >= 1 && index <= tasks.len() {
            tasks.remove(index - 1);
            println!("Task deleted!");
            return;
        }
    }

    println!("Invalid task number.");
}

fn clear_completed(tasks: &mut Vec<Task>) {
    tasks.retain(|t| !matches!(t.status, TaskStatus::Completed));
    println!("All completed tasks removed.");
}

fn search_tasks(tasks: &Vec<Task>) {
    let mut query = String::new();

    print!("Search for: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut query).unwrap();

    let query = query.trim().to_lowercase();

    println!("\n=== SEARCH RESULTS ===");

    let mut found = false;

    for (i, task) in tasks.iter().enumerate() {
        if task.description.to_lowercase().contains(&query) {
            println!(
                "{}. [{}] {} ({})",
                i + 1,
                task.status.as_str(),
                task.description,
                task.date
            );
            found = true;
        }
    }

    if !found {
        println!("No matching tasks found.");
    }
}

// ===== Helper =====

fn today_string() -> String {
    // No external crates — simple placeholder date.
    // You can change this manually if needed.
    "2025-11-14".to_string()
}

// ===== Main Program =====

fn main() {
    let filename = "tasks.txt";
    let mut tasks = load_tasks(filename).expect("Failed to load tasks");

    loop {
        println!("\n=== TASK MANAGER ===");
        println!("1. Add task");
        println!("2. List all tasks");
        println!("3. List pending tasks");
        println!("4. List completed tasks");
        println!("5. Mark task completed");
        println!("6. Search tasks");
        println!("7. Delete task");
        println!("8. Clear completed tasks");
        println!("9. Quit");
        print!("Enter choice: ");
        io::stdout().flush().unwrap();

        let mut choice = String::new();
        io::stdin().read_line(&mut choice).unwrap();

        match choice.trim() {
            "1" => add_task(&mut tasks),
            "2" => list_tasks(&tasks, "All"),
            "3" => list_tasks(&tasks, "Pending"),
            "4" => list_tasks(&tasks, "Completed"),
            "5" => mark_completed(&mut tasks),
            "6" => search_tasks(&tasks),
            "7" => delete_task(&mut tasks),
            "8" => clear_completed(&mut tasks),
            "9" => {
                println!("Saving tasks...");
                save_tasks(filename, &tasks).expect("Failed to save");
                println!("Goodbye!");
                break;
            }
            _ => println!("Invalid choice."),
        }
    }
}
