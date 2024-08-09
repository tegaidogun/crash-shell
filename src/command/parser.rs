use std::env;

pub struct Command {
    pub name: String,
    pub args: Vec<String>,
}

pub fn parse(input: &str) -> Result<Vec<Command>, &'static str> {
    let mut commands = Vec::new();
    let mut command_parts = Vec::new();
    let mut current_arg = String::new();
    let mut in_single_quotes = false;
    let mut in_double_quotes = false;
    let mut escape = false;

    let chars: Vec<char> = input.chars().collect();
    let mut i = 0;
    while i < chars.len() {
        match chars[i] {
            '\\' if !in_single_quotes => {
                escape = true;
                i += 1; // Skip the next character
                if i < chars.len() {
                    current_arg.push(chars[i]); // Append the escaped character
                }
            },
            '\'' if !escape && !in_double_quotes => {
                in_single_quotes = !in_single_quotes;
            },
            '"' if !escape && !in_single_quotes => {
                in_double_quotes = !in_double_quotes;
            },
            ' ' if !in_single_quotes && !in_double_quotes && !escape => {
                if !current_arg.is_empty() {
                    command_parts.push(expand_variables(&current_arg));
                    current_arg.clear();
                }
            },
            '&' if !in_single_quotes && !in_double_quotes && !escape => {
                if i + 1 < chars.len() && chars[i + 1] == '&' {
                    // End of a command, push to commands vector
                    if !current_arg.is_empty() {
                        command_parts.push(expand_variables(&current_arg));
                        current_arg.clear();
                    }
                    if !command_parts.is_empty() {
                        commands.push(Command {
                            name: command_parts.remove(0),
                            args: command_parts.clone(),
                        });
                        command_parts.clear();
                    }
                    i += 1; // Skip the next '&'
                }
            },
            _ if !escape => current_arg.push(chars[i]),
            _ => escape = false,
        }
        i += 1;
    }

    if !current_arg.is_empty() {
        command_parts.push(expand_variables(&current_arg));
    }
    if !command_parts.is_empty() {
        commands.push(Command {
            name: command_parts.remove(0),
            args: command_parts,
        });
    }

    Ok(commands)
}

fn expand_variables(arg: &str) -> String {
    let mut new_arg = String::new();
    let mut chars = arg.chars().peekable();
    while let Some(ch) = chars.next() {
        if ch == '~' {
            new_arg += &env::var("HOME").unwrap_or_else(|_| String::from(""));
        } else if ch == '$' {
            if let Some('{') = chars.peek() {
                let mut var_name = String::new();
                chars.next(); // Skip '{'
                while let Some(&ch) = chars.peek() {
                    if ch == '}' {
                        chars.next(); // Skip '}'
                        break;
                    } else {
                        var_name.push(ch);
                        chars.next();
                    }
                }
                new_arg += &env::var(&var_name).unwrap_or_default();
            } else {
                let mut var_name = String::new();
                while let Some(&ch) = chars.peek() {
                    if ch.is_alphanumeric() || ch == '_' {
                        var_name.push(ch);
                        chars.next();
                    } else {
                        break;
                    }
                }
                new_arg += &env::var(&var_name).unwrap_or_default();
            }
        } else {
            new_arg.push(ch);
        }
    }
    new_arg
}
