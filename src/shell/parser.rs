/// Command parser
use alloc::string::{String, ToString};
use alloc::vec::Vec;
use super::Command;

pub fn parse(input: &str) -> Command {
    let parts: Vec<&str> = input.split_whitespace().collect();
    
    if parts.is_empty() {
        return Command {
            name: String::new(),
            args: Vec::new(),
        };
    }

    let name = String::from(parts[0]);
    let args = parts[1..]
        .iter()
        .map(|s| String::from(*s))
        .collect();

    Command { name, args }
}
