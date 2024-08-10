use std::io::{self, Write};
use std::path::PathBuf;
use dirs::home_dir;
use crate::command::{executor, parser};

pub struct Shell {
    current_dir: PathBuf,
    local_paths: Vec<PathBuf>,
}

impl Shell {
    pub fn new() -> Shell {
        let home_dir = home_dir().unwrap_or_else(|| PathBuf::from("/"));
        let local_paths = vec![
            home_dir.join(".local/bin"),
            home_dir.join("bin"),
            home_dir.join(".bin"),
        ];

        Shell {
            current_dir: std::env::current_dir().unwrap(),
            local_paths,
        }
    }

    pub fn run(&mut self) {
        let mut input = String::new();
        self.add_local_paths_to_env();

        loop {
            print!("$ ");
            io::stdout().flush().unwrap();
            input.clear();
            if let Ok(_) = io::stdin().read_line(&mut input) {
                let trimmed_input = input.trim_end();

                if trimmed_input == "exit" {
                    break;
                }

                match parser::parse(trimmed_input) {
                    Ok(commands) => {
                        for command in commands {
                            if let Some(new_dir) = self.attempt_directory_change(&command) {
                                self.change_directory(new_dir);
                            } else {
                                // Execute the command using the executor module
                                if !executor::execute_command(&command, &self.current_dir) {
                                    eprintln!("Command execution failed");
                                }
                            }
                        }
                    },
                    Err(e) => println!("Error: {}", e),
                }
            }
        }
    }

    fn add_local_paths_to_env(&self) {
        if let Ok(mut path_var) = std::env::var("PATH") {
            for local_path in &self.local_paths {
                if local_path.exists() {
                    path_var.push_str(":");
                    path_var.push_str(local_path.to_str().unwrap());
                }
            }
            std::env::set_var("PATH", path_var);
        }
    }

    fn attempt_directory_change(&self, command: &parser::Command) -> Option<PathBuf> {
        if command.args.len() == 1 {
            let arg = &command.args[0];
            let path = PathBuf::from(arg);
            if path.is_dir() {
                return Some(path);
            }
        }
        None
    }

    fn change_directory(&mut self, new_dir: PathBuf) {
        if let Ok(canonicalized_path) = new_dir.canonicalize() {
            if std::env::set_current_dir(&canonicalized_path).is_ok() {
                self.current_dir = canonicalized_path;
                println!("Changed directory to: {}", self.current_dir.display());
            } else {
                eprintln!("Failed to change directory");
            }
        } else {
            eprintln!("Failed to canonicalize path");
        }
    }
}
