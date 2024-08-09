use std::io::{self, Write};
use std::path::PathBuf;
use std::ffi::CString;
use libc::chdir;
use crate::command::{executor, parser};  // Import executor and parser modules

pub struct Shell {
    current_dir: PathBuf,
}

impl Shell {
    pub fn new() -> Shell {
        Shell {
            current_dir: std::env::current_dir().unwrap(),
        }
    }

    pub fn run(&mut self) {
        let mut input = String::new();
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

    fn attempt_directory_change(&self, command: &parser::Command) -> Option<PathBuf> {
        if command.args.len() == 1 {
            let arg = &command.args[0];
            if let Ok(path) = PathBuf::from(arg).canonicalize() {
                if path.is_dir() {
                    return Some(path);
                }
            }
        }
        None
    }

    fn change_directory(&mut self, new_dir: PathBuf) {
        let new_path_cstr = CString::new(new_dir.to_str().unwrap()).expect("Path conversion failed");
        unsafe {
            if chdir(new_path_cstr.as_ptr()) == 0 {
                self.current_dir = new_dir;
                println!("Changed directory to: {}", self.current_dir.display());
            } else {
                eprintln!("Failed to change directory");
            }
        }
    }
}
