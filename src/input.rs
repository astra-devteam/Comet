//! Simple keyboard input handler for the shell

use alloc::string::String;

/// Keyboard state
pub struct KeyboardHandler {
    buffer: String,
    shift_pressed: bool,
    ctrl_pressed: bool,
}

impl KeyboardHandler {
    pub fn new() -> Self {
        KeyboardHandler {
            buffer: String::new(),
            shift_pressed: false,
            ctrl_pressed: false,
        }
    }

    pub fn handle_keystroke(&mut self, scancode: u8) -> Option<String> {
        match scancode {
            // Spacebar
            0x39 => self.buffer.push(' '),
            // Backspace
            0x0E => {
                self.buffer.pop();
            }
            // Enter - return input
            0x1C => {
                let input = self.buffer.clone();
                self.buffer.clear();
                return Some(input);
            }
            // Shift
            0x2A | 0x36 => self.shift_pressed = true,
            // Ctrl
            0x1D => self.ctrl_pressed = true,
            // A-Z keys
            0x1E..=0x54 => {
                let ch = self.scancode_to_char(scancode);
                self.buffer.push(ch);
            }
            _ => {}
        }
        None
    }

    fn scancode_to_char(&self, scancode: u8) -> char {
        let chars = if self.shift_pressed {
            "ABCDEFGHIJKLMNOPQRSTUVWXYZ"
        } else {
            "abcdefghijklmnopqrstuvwxyz"
        };

        match scancode {
            0x1E..=0x54 => {
                let idx = (scancode - 0x1E) as usize;
                if idx < chars.len() {
                    chars.chars().nth(idx).unwrap_or('?')
                } else {
                    '?'
                }
            }
            _ => '?',
        }
    }

    pub fn get_buffer(&self) -> &str {
        &self.buffer
    }
}
