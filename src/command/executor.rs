use super::parser::Command;
use std::process::Command as SystemCommand;
use std::path::Path;

// Updated to return a boolean indicating success or failure
pub fn execute_command(command: &Command, current_dir: &Path) -> bool {
    let output = SystemCommand::new(&command.name)
        .args(&command.args)
        .current_dir(current_dir)
        .output();

    match output {
        Ok(output) => {
            if output.status.success() {
                let stdout = String::from_utf8_lossy(&output.stdout);
                print!("{}", stdout);
                true  // Return true if command executed successfully
            } else {
                let stderr = String::from_utf8_lossy(&output.stderr);
                eprintln!("Command failed: {}", stderr);
                false  // Return false if command failed
            }
        },
        Err(e) => {
            eprintln!("Command execution error: {}", e);
            false  // Return false if there was an error executing the command
        }
    }
}
