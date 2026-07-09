/// Graphical User Interface Module
pub mod window;
pub mod widget;

use crate::vga::writer::{VgaWriter, Color};
use alloc::vec::Vec;
use alloc::string::String;

/// GUI coordinate
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Point {
    pub x: u16,
    pub y: u16,
}

/// GUI rectangle
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Rect {
    pub x: u16,
    pub y: u16,
    pub width: u16,
    pub height: u16,
}

impl Rect {
    pub fn new(x: u16, y: u16, width: u16, height: u16) -> Self {
        Rect { x, y, width, height }
    }

    pub fn contains(&self, point: Point) -> bool {
        point.x >= self.x
            && point.x < self.x + self.width
            && point.y >= self.y
            && point.y < self.y + self.height
    }
}

/// Simple GUI manager
pub struct GUI {
    windows: Vec<window::Window>,
    focused_window: Option<usize>,
}

impl GUI {
    pub fn new() -> Self {
        GUI {
            windows: Vec::new(),
            focused_window: None,
        }
    }

    pub fn create_window(&mut self, title: &str, rect: Rect) -> usize {
        let win = window::Window::new(String::from(title), rect);
        self.windows.push(win);
        let idx = self.windows.len() - 1;
        self.focused_window = Some(idx);
        idx
    }

    pub fn draw_all(&self) {
        let mut vga = VgaWriter::new();
        
        for (idx, window) in self.windows.iter().enumerate() {
            let is_focused = self.focused_window == Some(idx);
            window.draw(&mut vga, is_focused);
        }
    }

    pub fn handle_click(&mut self, point: Point) {
        for (idx, window) in self.windows.iter().enumerate() {
            if window.bounds.contains(point) {
                self.focused_window = Some(idx);
                break;
            }
        }
    }
}
