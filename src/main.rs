mod shell;
mod command;
mod error;

use shell::Shell;

fn main() {
    let mut shell = Shell::new();
    shell.run();
}
