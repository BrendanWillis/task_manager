# Rust Task Manager

A simple command-line Task Manager written in Rust.
The program allows users to add tasks, list tasks, mark tasks as completed, delete tasks, search tasks, and save/load all data from a text file.
This project demonstrates core Rust concepts such as variables, loops, functions, vectors, pattern matching, structs, enums, and Result-based error handling.
Add a description of your project here.


## Instructions for Build and Use

Steps to build and/or run the software:

1. Install Rust using rustup (https://rustup.rs/
)
2. Clone or download this repository
3. Navigate into the project folder
 ```
 cd task_manager
 ```
4. Build and run the program
```
cargo run
```

Instructions for using the software:

1. Choose a number from the menu to perform an action

2. Add tasks by typing descriptions when prompted

3. View tasks (all, pending, or completed)

4. Mark tasks as complete or delete them

5. Search tasks by keyword

6. Quit the program to automatically save your work

A file named **tasks.txt** is created to store your task list.


## Development Environment 

To recreate the development environment, you need the following software and/or libraries with the specified versions:

To recreate the development environment, install:

* Rust (1.70+ recommended) – Installed via rustup

* Cargo – Included automatically with Rust

* VS Code (optional but recommended)

* Rust Analyzer extension (for VS Code)

* No external crates or libraries are required.

## Useful Websites to Learn More

I found these websites useful in developing this software:

* [The Rust Book](https://doc.rust-lang.org/book/)

* [Rust Standard Library Documentation](https://doc.rust-lang.org/std/)

* [Rust By Example](https://doc.rust-lang.org/rust-by-example/)

## Future Work

The following items I plan to fix, improve, and/or add to this project in the future:

* [Use real system date/time instead of a placeholder]
* [Add editing of existing task descriptions]