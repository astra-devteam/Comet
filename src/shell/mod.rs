/// Command Line Interface / Shell Module
pub mod parser;
pub mod commands;

use alloc::string::{String, ToString};
use alloc::vec::Vec;
use crate::vga::writer::{VgaWriter, Color};

/// Shell prompt structure
pub struct Shell {
    prompt: String,
    history: Vec<String>,
    current_user: String,
    current_dir: String,
}

impl Shell {
    pub fn new() -> Self {
        Shell {
            prompt: String::from("comet> "),
            history: Vec::new(),
            current_user: String::from("root"),
            current_dir: String::from("/"),
        }
    }

    pub fn display_prompt(&self) {
        let mut vga = VgaWriter::new();
        vga.set_color(Color::Green, Color::Black);
        vga.write_str(&self.current_user);
        vga.write_str("@comet:");
        vga.set_color(Color::Blue, Color::Black);
        vga.write_str(&self.current_dir);
        vga.set_color(Color::Green, Color::Black);
        vga.write_str("$ ");
        vga.set_color(Color::White, Color::Black);
    }

    pub fn parse_command(&mut self, input: &str) -> Command {
        let trimmed = input.trim();
        self.history.push(String::from(trimmed));
        
        parser::parse(trimmed)
    }

    pub fn execute_command(&mut self, cmd: Command) -> CommandResult {
        commands::execute(cmd, self)
    }

    pub fn change_directory(&mut self, path: &str) {
        if path == ".." {
            if self.current_dir.len() > 1 {
                if let Some(pos) = self.current_dir.rfind('/') {
                    if pos > 0 {
                        self.current_dir.truncate(pos);
                    } else {
                        self.current_dir = String::from("/");
                    }
                }
            }
        } else if path.starts_with('/') {
            self.current_dir = String::from(path);
        } else {
            if !self.current_dir.ends_with('/') {
                self.current_dir.push('/');
            }
            self.current_dir.push_str(path);
        }
    }
}

/// Command structure
#[derive(Debug, Clone)]
pub struct Command {
    pub name: String,
    pub args: Vec<String>,
}

/// Command execution result
#[derive(Debug, Clone)]
pub enum CommandResult {
    Success(String),
    Error(String),
    Exit,
}
