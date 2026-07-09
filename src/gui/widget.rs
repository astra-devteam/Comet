/// GUI Widgets
use crate::vga::writer::{VgaWriter, Color};
use alloc::string::String;
use super::Rect;

/// Widget types
#[derive(Debug, Clone)]
pub enum WidgetType {
    Label(String),
    Button(String),
    TextBox(String),
    ProgressBar { value: u32, max: u32 },
}

/// Widget structure
#[derive(Debug, Clone)]
pub struct Widget {
    pub bounds: Rect,
    pub widget_type: WidgetType,
}

impl Widget {
    pub fn label(x: u16, y: u16, text: &str) -> Self {
        Widget {
            bounds: Rect::new(x, y, text.len() as u16, 1),
            widget_type: WidgetType::Label(String::from(text)),
        }
    }

    pub fn button(x: u16, y: u16, label: &str) -> Self {
        let width = label.len() as u16 + 4; // Add padding
        Widget {
            bounds: Rect::new(x, y, width, 3),
            widget_type: WidgetType::Button(String::from(label)),
        }
    }

    pub fn textbox(x: u16, y: u16, width: u16) -> Self {
        Widget {
            bounds: Rect::new(x, y, width, 1),
            widget_type: WidgetType::TextBox(String::new()),
        }
    }

    pub fn progress_bar(x: u16, y: u16, width: u16, value: u32, max: u32) -> Self {
        Widget {
            bounds: Rect::new(x, y, width, 1),
            widget_type: WidgetType::ProgressBar { value, max },
        }
    }

    pub fn draw(&self, vga: &mut VgaWriter) {
        match &self.widget_type {
            WidgetType::Label(text) => {
                vga.write_str_at(text, self.bounds.x, self.bounds.y);
            }
            WidgetType::Button(label) => {
                vga.set_color(Color::Black, Color::White);
                vga.write_str_at(&format!(" {} ", label), self.bounds.x, self.bounds.y + 1);
                vga.set_color(Color::White, Color::Black);
            }
            WidgetType::TextBox(text) => {
                vga.set_color(Color::Black, Color::White);
                let padded = format!("{:<width$}", text, width = self.bounds.width as usize);
                vga.write_str_at(&padded, self.bounds.x, self.bounds.y);
                vga.set_color(Color::White, Color::Black);
            }
            WidgetType::ProgressBar { value, max } => {
                let filled = ((*value as u16 * self.bounds.width) / *max as u16).min(self.bounds.width);
                vga.set_color(Color::Green, Color::Black);
                for x in 0..filled {
                    vga.write_char_at('█', self.bounds.x + x, self.bounds.y);
                }
                vga.set_color(Color::DarkGray, Color::Black);
                for x in filled..self.bounds.width {
                    vga.write_char_at('░', self.bounds.x + x, self.bounds.y);
                }
                vga.set_color(Color::White, Color::Black);
            }
        }
    }
}
