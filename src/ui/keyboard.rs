use ratatui::{
    buffer::Buffer,
    layout::Rect,
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::Widget,
};

use crate::keyboard::{Hand, Layout};

/// Renders the Kinesis 360 keyboard layout with finger coloring and key highlighting.
pub struct KeyboardWidget<'a> {
    layout: &'a Layout,
    /// Character to highlight as the "next key to press"
    pub highlight_char: Option<char>,
    /// Character that was just pressed (for flash feedback)
    pub last_pressed: Option<(char, bool)>, // (char, was_correct)
}

impl<'a> KeyboardWidget<'a> {
    pub fn new(layout: &'a Layout) -> Self {
        Self {
            layout,
            highlight_char: None,
            last_pressed: None,
        }
    }
}

impl Widget for KeyboardWidget<'_> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        if area.height < 8 || area.width < 50 {
            return;
        }

        let gap = 4u16; // gap between left and right halves
        let key_w = 4u16; // width per key cell

        // Compute centering offset
        let total_keyboard_width = 6 * key_w + gap + 6 * key_w;
        let x_offset = area.x + area.width.saturating_sub(total_keyboard_width) / 2;

        // Render main well (rows 0-4)
        for key in &self.layout.keys {
            if key.row > 4 {
                continue; // thumb cluster handled separately
            }

            let (base_x, col) = match key.side {
                Hand::Left => (x_offset, key.col as u16),
                Hand::Right => (x_offset + 6 * key_w + gap, key.col as u16),
            };

            let x = base_x + col * key_w;
            let y = area.y + key.row as u16;

            if y >= area.y + area.height || x + key_w > area.x + area.width {
                continue;
            }

            let mut style = Style::default().fg(key.finger.color());

            // Home row keys get a subtle indicator
            if key.is_home {
                style = style.add_modifier(Modifier::UNDERLINED);
            }

            // Highlight the target key
            if let Some(target) = self.highlight_char {
                let is_target = key.char_unshifted == Some(target)
                    || key.char_shifted == Some(target);
                if is_target {
                    style = Style::default()
                        .fg(Color::Black)
                        .bg(key.finger.color())
                        .add_modifier(Modifier::BOLD);
                }
            }

            // Flash feedback for last pressed key
            if let Some((pressed, correct)) = self.last_pressed {
                let is_pressed = key.char_unshifted == Some(pressed)
                    || key.char_shifted == Some(pressed);
                if is_pressed {
                    let bg = if correct { Color::Green } else { Color::Red };
                    style = Style::default().fg(Color::Black).bg(bg).add_modifier(Modifier::BOLD);
                }
            }

            // Truncate label to fit
            let label = if key.label.len() <= key_w as usize {
                format!("{:^width$}", key.label, width = key_w as usize)
            } else {
                key.label[..key_w as usize].to_string()
            };

            buf.set_string(x, y, &label, style);
        }

        // Render thumb clusters (rows 5-7) below main well
        let thumb_y_start = area.y + 6; // one line gap after row 4
        if thumb_y_start + 2 >= area.y + area.height {
            return;
        }

        for key in &self.layout.keys {
            if key.row < 5 {
                continue;
            }

            let thumb_row = key.row - 5;
            let (base_x, col) = match key.side {
                Hand::Left => (x_offset + 2 * key_w, key.col as u16),
                Hand::Right => (x_offset + 6 * key_w + gap + 2 * key_w, key.col as u16),
            };

            let x = base_x + col * key_w;
            let y = thumb_y_start + thumb_row as u16;

            if y >= area.y + area.height || x + key_w > area.x + area.width {
                continue;
            }

            let style = Style::default().fg(key.finger.color()).add_modifier(Modifier::DIM);
            let label = format!("{:^width$}", key.label, width = key_w as usize);
            buf.set_string(x, y, &label, style);
        }

        // Draw the gap separator
        let sep_x = x_offset + 6 * key_w;
        for row in 0..5u16 {
            let y = area.y + row;
            if y < area.y + area.height {
                buf.set_string(
                    sep_x,
                    y,
                    &" ".repeat(gap as usize),
                    Style::default(),
                );
            }
        }
    }
}

/// Render a finger hint line below the keyboard.
pub fn finger_hint_line(layout: &Layout, c: char) -> Line<'static> {
    if let Some(key) = layout.key_for_char(c) {
        let finger_label = key.finger.label();
        let color = key.finger.color();
        let needs_shift = layout.needs_shift(c);

        let mut spans = vec![
            Span::styled("  Finger: ", Style::default().fg(Color::DarkGray)),
            Span::styled(finger_label, Style::default().fg(color).add_modifier(Modifier::BOLD)),
        ];
        if needs_shift {
            let shift_side = match key.side {
                // Use opposite hand's shift
                Hand::Left => "R Shift",
                Hand::Right => "L Shift",
            };
            spans.push(Span::styled(" + ", Style::default().fg(Color::DarkGray)));
            spans.push(Span::styled(shift_side, Style::default().fg(Color::Red)));
        }
        Line::from(spans)
    } else {
        Line::from("")
    }
}
