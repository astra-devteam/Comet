/// Window management
use super::{Rect, Point};
use crate::vga::writer::{VgaWriter, Color};
use alloc::string::String;
use alloc::vec::Vec;
use super::widget::Widget;

/// Window structure
pub struct Window {
    pub title: String,
    pub bounds: Rect,
    pub widgets: Vec<Widget>,
    pub visible: bool,
}

impl Window {
    pub fn new(title: String, bounds: Rect) -> Self {
        Window {
            title,
            bounds,
            widgets: Vec::new(),
            visible: true,
        }
    }

    pub fn draw(&self, vga: &mut VgaWriter, focused: bool) {
        if !self.visible {
            return;
        }

        // Draw window border
        let border_color = if focused { Color::White } else { Color::LightGray };
        vga.set_color(border_color, Color::Black);

        // Top border
        for x in self.bounds.x..self.bounds.x + self.bounds.width {
            vga.write_char_at('─', x, self.bounds.y);
        }

        // Side borders
        for y in self.bounds.y + 1..self.bounds.y + self.bounds.height {
            vga.write_char_at('│', self.bounds.x, y);
            vga.write_char_at('│', self.bounds.x + self.bounds.width - 1, y);
        }

        // Bottom border
        for x in self.bounds.x..self.bounds.x + self.bounds.width {
            vga.write_char_at('─', x, self.bounds.y + self.bounds.height - 1);
        }

        // Corners
        vga.write_char_at('┌', self.bounds.x, self.bounds.y);
        vga.write_char_at('┐', self.bounds.x + self.bounds.width - 1, self.bounds.y);
        vga.write_char_at('└', self.bounds.x, self.bounds.y + self.bounds.height - 1);
        vga.write_char_at('┘', self.bounds.x + self.bounds.width - 1, self.bounds.y + self.bounds.height - 1);

        // Draw title bar
        vga.set_color(Color::Black, if focused { Color::White } else { Color::LightGray });
        vga.write_str_at(&format!(" {} ", self.title), self.bounds.x + 2, self.bounds.y);

        // Draw widgets
        vga.set_color(Color::White, Color::Black);
        for widget in &self.widgets {
            widget.draw(vga);
        }
    }

    pub fn add_widget(&mut self, widget: Widget) {
        self.widgets.push(widget);
    }
}
