use ratatui::{
    buffer::Buffer,
    layout::Rect,
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::Widget,
};

use crate::keyboard::Layout;

/// Renders the Kinesis 360 keyboard with [bracketed] keys, finger colors, and highlighting.
pub struct KeyboardWidget<'a> {
    layout: &'a Layout,
    pub highlight_char: Option<char>,
    pub last_pressed: Option<(char, bool)>,
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

/// Visual row definition: (left_keys, right_keys)
/// None = empty cell (modifier position, keeps alignment)
type KeyRow = (Vec<Option<char>>, Vec<Option<char>>);

fn keyboard_rows() -> Vec<KeyRow> {
    vec![
        // Row 0: Number row
        (
            vec![Some('='), Some('1'), Some('2'), Some('3'), Some('4'), Some('5')],
            vec![Some('6'), Some('7'), Some('8'), Some('9'), Some('0'), Some('-')],
        ),
        // Row 1: Top alpha (col 0 = Tab, skipped)
        (
            vec![None, Some('q'), Some('w'), Some('e'), Some('r'), Some('t')],
            vec![Some('y'), Some('u'), Some('i'), Some('o'), Some('p'), Some('\\')],
        ),
        // Row 2: Home row (col 0 = Esc, skipped)
        (
            vec![None, Some('a'), Some('s'), Some('d'), Some('f'), Some('g')],
            vec![Some('h'), Some('j'), Some('k'), Some('l'), Some(';'), Some('\'')],
        ),
        // Row 3: Bottom (col 0 = Shift, skipped; right col 5 = Shift, skipped)
        (
            vec![None, Some('z'), Some('x'), Some('c'), Some('v'), Some('b')],
            vec![Some('n'), Some('m'), Some(','), Some('.'), Some('/'), None],
        ),
    ]
}

impl Widget for KeyboardWidget<'_> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        if area.height < 5 || area.width < 44 {
            return;
        }

        let rows = keyboard_rows();
        let key_w = 3u16; // [X] = 3 chars
        let left_cols = 6u16;
        let right_cols = 6u16;
        let gap = 6u16;
        let total_w = left_cols * key_w + gap + right_cols * key_w;
        let x_base = area.x + area.width.saturating_sub(total_w) / 2;
        let right_base = x_base + left_cols * key_w + gap;

        let bracket_style = Style::default().fg(Color::DarkGray);

        // Render main well rows
        for (row_idx, (left, right)) in rows.iter().enumerate() {
            let y = area.y + row_idx as u16;
            if y >= area.y + area.height {
                break;
            }

            for (col, key_opt) in left.iter().enumerate() {
                let x = x_base + col as u16 * key_w;
                if let Some(c) = key_opt {
                    self.render_key(buf, x, y, *c, bracket_style);
                }
            }

            for (col, key_opt) in right.iter().enumerate() {
                let x = right_base + col as u16 * key_w;
                if let Some(c) = key_opt {
                    self.render_key(buf, x, y, *c, bracket_style);
                }
            }
        }

        // Thumb row: [Bk] on left, [Sp] on right
        let thumb_y = area.y + rows.len() as u16;
        if thumb_y < area.y + area.height {
            // Backspace: centered under left index fingers (cols 4-5)
            let bk_x = x_base + 4 * key_w;
            let bk_style = Style::default()
                .fg(Color::Blue)
                .add_modifier(Modifier::DIM);
            buf.set_string(bk_x, thumb_y, "[", bracket_style);
            buf.set_string(bk_x + 1, thumb_y, "Bk", bk_style);
            buf.set_string(bk_x + 3, thumb_y, "]", bracket_style);

            // Space: centered under right index fingers (cols 0-1)
            let sp_x = right_base + 1 * key_w;
            let sp_style = self.key_style(' ', bracket_style);
            let sp_bracket = self.bracket_style(' ', bracket_style);
            buf.set_string(sp_x, thumb_y, "[", sp_bracket);
            buf.set_string(sp_x + 1, thumb_y, "Sp", sp_style);
            buf.set_string(sp_x + 3, thumb_y, "]", sp_bracket);
        }
    }
}

impl KeyboardWidget<'_> {
    fn render_key(&self, buf: &mut Buffer, x: u16, y: u16, c: char, bracket_style: Style) {
        let label = if c.is_ascii_lowercase() {
            c.to_ascii_uppercase().to_string()
        } else {
            c.to_string()
        };

        let key_style = self.key_style(c, bracket_style);
        let br_style = self.bracket_style(c, bracket_style);

        buf.set_string(x, y, "[", br_style);
        buf.set_string(x + 1, y, &label, key_style);
        buf.set_string(x + 2, y, "]", br_style);
    }

    fn key_style(&self, c: char, _bracket_style: Style) -> Style {
        let key_def = self.layout.key_for_char(c);
        let finger_color = key_def.map(|k| k.finger.color()).unwrap_or(Color::White);
        let is_home = key_def.map(|k| k.is_home).unwrap_or(false);

        // Check flash feedback first (highest priority)
        if let Some((pressed, correct)) = self.last_pressed {
            if pressed == c {
                let bg = if correct { Color::Green } else { Color::Red };
                return Style::default()
                    .fg(Color::Black)
                    .bg(bg)
                    .add_modifier(Modifier::BOLD);
            }
        }

        // Check highlight (target key)
        if self.highlight_char == Some(c) {
            return Style::default()
                .fg(Color::Black)
                .bg(finger_color)
                .add_modifier(Modifier::BOLD);
        }

        // Default: dim with finger color, home row slightly brighter
        let mut style = Style::default().fg(finger_color);
        if !is_home {
            style = style.add_modifier(Modifier::DIM);
        }
        style
    }

    fn bracket_style(&self, c: char, default: Style) -> Style {
        // Flash feedback
        if let Some((pressed, correct)) = self.last_pressed {
            if pressed == c {
                let color = if correct { Color::Green } else { Color::Red };
                return Style::default().fg(color);
            }
        }

        // Highlight
        if self.highlight_char == Some(c) {
            let key_def = self.layout.key_for_char(c);
            let finger_color = key_def.map(|k| k.finger.color()).unwrap_or(Color::White);
            return Style::default().fg(finger_color);
        }

        default
    }
}

/// Render a finger hint line below the keyboard.
pub fn finger_hint_line(layout: &Layout, c: char) -> Line<'static> {
    if let Some(key) = layout.key_for_char(c) {
        let finger_label = key.finger.label();
        let color = key.finger.color();
        let needs_shift = layout.needs_shift(c);

        let mut spans = vec![
            Span::styled("Finger: ", Style::default().fg(Color::DarkGray)),
            Span::styled(
                finger_label,
                Style::default().fg(color).add_modifier(Modifier::BOLD),
            ),
        ];
        if needs_shift {
            let shift_side = match key.side {
                crate::keyboard::Hand::Left => "R Shift",
                crate::keyboard::Hand::Right => "L Shift",
            };
            spans.push(Span::styled(" + ", Style::default().fg(Color::DarkGray)));
            spans.push(Span::styled(
                shift_side,
                Style::default().fg(Color::Magenta),
            ));
        }
        Line::from(spans)
    } else if c == ' ' {
        Line::from(vec![
            Span::styled("Finger: ", Style::default().fg(Color::DarkGray)),
            Span::styled(
                "R Thumb",
                Style::default()
                    .fg(Color::Blue)
                    .add_modifier(Modifier::BOLD),
            ),
            Span::styled(" (Space)", Style::default().fg(Color::DarkGray)),
        ])
    } else {
        Line::from("")
    }
}
