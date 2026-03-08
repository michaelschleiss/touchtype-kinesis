use super::finger::*;
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct KeyDef {
    pub label: &'static str,
    pub char_unshifted: Option<char>,
    pub char_shifted: Option<char>,
    pub finger: Finger,
    /// Row in the rendering grid (0-4 = main well, 5-7 = thumb cluster)
    pub row: u8,
    /// Column in the rendering grid
    pub col: u8,
    pub side: Hand,
    pub is_home: bool,
}

pub struct Layout {
    pub keys: Vec<KeyDef>,
    char_to_key: HashMap<char, usize>,
}

impl Layout {
    pub fn kinesis360() -> Self {
        let keys = vec![
            // ═══════════════════════════════════════
            // LEFT HALF — Main Well
            // ═══════════════════════════════════════

            // Row 0: Number row
            KeyDef { label: "=+",  char_unshifted: Some('='), char_shifted: Some('+'), finger: L_PINKY,  row: 0, col: 0, side: Hand::Left, is_home: false },
            KeyDef { label: "1!",  char_unshifted: Some('1'), char_shifted: Some('!'), finger: L_PINKY,  row: 0, col: 1, side: Hand::Left, is_home: false },
            KeyDef { label: "2@",  char_unshifted: Some('2'), char_shifted: Some('@'), finger: L_RING,   row: 0, col: 2, side: Hand::Left, is_home: false },
            KeyDef { label: "3#",  char_unshifted: Some('3'), char_shifted: Some('#'), finger: L_MIDDLE, row: 0, col: 3, side: Hand::Left, is_home: false },
            KeyDef { label: "4$",  char_unshifted: Some('4'), char_shifted: Some('$'), finger: L_INDEX,  row: 0, col: 4, side: Hand::Left, is_home: false },
            KeyDef { label: "5%",  char_unshifted: Some('5'), char_shifted: Some('%'), finger: L_INDEX,  row: 0, col: 5, side: Hand::Left, is_home: false },

            // Row 1: Top alpha
            KeyDef { label: "Tab", char_unshifted: None,      char_shifted: None,      finger: L_PINKY,  row: 1, col: 0, side: Hand::Left, is_home: false },
            KeyDef { label: "Q",   char_unshifted: Some('q'), char_shifted: Some('Q'), finger: L_PINKY,  row: 1, col: 1, side: Hand::Left, is_home: false },
            KeyDef { label: "W",   char_unshifted: Some('w'), char_shifted: Some('W'), finger: L_RING,   row: 1, col: 2, side: Hand::Left, is_home: false },
            KeyDef { label: "E",   char_unshifted: Some('e'), char_shifted: Some('E'), finger: L_MIDDLE, row: 1, col: 3, side: Hand::Left, is_home: false },
            KeyDef { label: "R",   char_unshifted: Some('r'), char_shifted: Some('R'), finger: L_INDEX,  row: 1, col: 4, side: Hand::Left, is_home: false },
            KeyDef { label: "T",   char_unshifted: Some('t'), char_shifted: Some('T'), finger: L_INDEX,  row: 1, col: 5, side: Hand::Left, is_home: false },

            // Row 2: Home row
            KeyDef { label: "Esc", char_unshifted: None,      char_shifted: None,      finger: L_PINKY,  row: 2, col: 0, side: Hand::Left, is_home: false },
            KeyDef { label: "A",   char_unshifted: Some('a'), char_shifted: Some('A'), finger: L_PINKY,  row: 2, col: 1, side: Hand::Left, is_home: true },
            KeyDef { label: "S",   char_unshifted: Some('s'), char_shifted: Some('S'), finger: L_RING,   row: 2, col: 2, side: Hand::Left, is_home: true },
            KeyDef { label: "D",   char_unshifted: Some('d'), char_shifted: Some('D'), finger: L_MIDDLE, row: 2, col: 3, side: Hand::Left, is_home: true },
            KeyDef { label: "F",   char_unshifted: Some('f'), char_shifted: Some('F'), finger: L_INDEX,  row: 2, col: 4, side: Hand::Left, is_home: true },
            KeyDef { label: "G",   char_unshifted: Some('g'), char_shifted: Some('G'), finger: L_INDEX,  row: 2, col: 5, side: Hand::Left, is_home: false },

            // Row 3: Bottom alpha
            KeyDef { label: "Shf", char_unshifted: None,      char_shifted: None,      finger: L_PINKY,  row: 3, col: 0, side: Hand::Left, is_home: false },
            KeyDef { label: "Z",   char_unshifted: Some('z'), char_shifted: Some('Z'), finger: L_PINKY,  row: 3, col: 1, side: Hand::Left, is_home: false },
            KeyDef { label: "X",   char_unshifted: Some('x'), char_shifted: Some('X'), finger: L_RING,   row: 3, col: 2, side: Hand::Left, is_home: false },
            KeyDef { label: "C",   char_unshifted: Some('c'), char_shifted: Some('C'), finger: L_MIDDLE, row: 3, col: 3, side: Hand::Left, is_home: false },
            KeyDef { label: "V",   char_unshifted: Some('v'), char_shifted: Some('V'), finger: L_INDEX,  row: 3, col: 4, side: Hand::Left, is_home: false },
            KeyDef { label: "B",   char_unshifted: Some('b'), char_shifted: Some('B'), finger: L_INDEX,  row: 3, col: 5, side: Hand::Left, is_home: false },

            // Row 4: Bottom row
            KeyDef { label: "Fn",  char_unshifted: None,      char_shifted: None,      finger: L_PINKY,  row: 4, col: 0, side: Hand::Left, is_home: false },
            KeyDef { label: "`~",  char_unshifted: Some('`'), char_shifted: Some('~'), finger: L_PINKY,  row: 4, col: 1, side: Hand::Left, is_home: false },

            // ═══════════════════════════════════════
            // LEFT THUMB CLUSTER
            // ═══════════════════════════════════════
            KeyDef { label: "Ctl", char_unshifted: None,      char_shifted: None,      finger: L_THUMB,  row: 5, col: 0, side: Hand::Left, is_home: false },
            KeyDef { label: "Alt", char_unshifted: None,      char_shifted: None,      finger: L_THUMB,  row: 5, col: 1, side: Hand::Left, is_home: false },
            KeyDef { label: "Bks", char_unshifted: None,      char_shifted: None,      finger: L_THUMB,  row: 6, col: 0, side: Hand::Left, is_home: false },
            KeyDef { label: "Del", char_unshifted: None,      char_shifted: None,      finger: L_THUMB,  row: 6, col: 1, side: Hand::Left, is_home: false },
            KeyDef { label: "Hom", char_unshifted: None,      char_shifted: None,      finger: L_THUMB,  row: 7, col: 0, side: Hand::Left, is_home: false },
            KeyDef { label: "End", char_unshifted: None,      char_shifted: None,      finger: L_THUMB,  row: 7, col: 1, side: Hand::Left, is_home: false },

            // ═══════════════════════════════════════
            // RIGHT HALF — Main Well
            // ═══════════════════════════════════════

            // Row 0: Number row
            KeyDef { label: "6^",  char_unshifted: Some('6'), char_shifted: Some('^'), finger: R_INDEX,  row: 0, col: 0, side: Hand::Right, is_home: false },
            KeyDef { label: "7&",  char_unshifted: Some('7'), char_shifted: Some('&'), finger: R_INDEX,  row: 0, col: 1, side: Hand::Right, is_home: false },
            KeyDef { label: "8*",  char_unshifted: Some('8'), char_shifted: Some('*'), finger: R_MIDDLE, row: 0, col: 2, side: Hand::Right, is_home: false },
            KeyDef { label: "9(",  char_unshifted: Some('9'), char_shifted: Some('('), finger: R_RING,   row: 0, col: 3, side: Hand::Right, is_home: false },
            KeyDef { label: "0)",  char_unshifted: Some('0'), char_shifted: Some(')'), finger: R_PINKY,  row: 0, col: 4, side: Hand::Right, is_home: false },
            KeyDef { label: "-_",  char_unshifted: Some('-'), char_shifted: Some('_'), finger: R_PINKY,  row: 0, col: 5, side: Hand::Right, is_home: false },

            // Row 1: Top alpha
            KeyDef { label: "Y",   char_unshifted: Some('y'), char_shifted: Some('Y'), finger: R_INDEX,  row: 1, col: 0, side: Hand::Right, is_home: false },
            KeyDef { label: "U",   char_unshifted: Some('u'), char_shifted: Some('U'), finger: R_INDEX,  row: 1, col: 1, side: Hand::Right, is_home: false },
            KeyDef { label: "I",   char_unshifted: Some('i'), char_shifted: Some('I'), finger: R_MIDDLE, row: 1, col: 2, side: Hand::Right, is_home: false },
            KeyDef { label: "O",   char_unshifted: Some('o'), char_shifted: Some('O'), finger: R_RING,   row: 1, col: 3, side: Hand::Right, is_home: false },
            KeyDef { label: "P",   char_unshifted: Some('p'), char_shifted: Some('P'), finger: R_PINKY,  row: 1, col: 4, side: Hand::Right, is_home: false },
            KeyDef { label: "\\|", char_unshifted: Some('\\'),char_shifted: Some('|'), finger: R_PINKY,  row: 1, col: 5, side: Hand::Right, is_home: false },

            // Row 2: Home row
            KeyDef { label: "H",   char_unshifted: Some('h'), char_shifted: Some('H'), finger: R_INDEX,  row: 2, col: 0, side: Hand::Right, is_home: false },
            KeyDef { label: "J",   char_unshifted: Some('j'), char_shifted: Some('J'), finger: R_INDEX,  row: 2, col: 1, side: Hand::Right, is_home: true },
            KeyDef { label: "K",   char_unshifted: Some('k'), char_shifted: Some('K'), finger: R_MIDDLE, row: 2, col: 2, side: Hand::Right, is_home: true },
            KeyDef { label: "L",   char_unshifted: Some('l'), char_shifted: Some('L'), finger: R_RING,   row: 2, col: 3, side: Hand::Right, is_home: true },
            KeyDef { label: ";:",  char_unshifted: Some(';'), char_shifted: Some(':'), finger: R_PINKY,  row: 2, col: 4, side: Hand::Right, is_home: true },
            KeyDef { label: "'\"", char_unshifted: Some('\''),char_shifted: Some('"'), finger: R_PINKY,  row: 2, col: 5, side: Hand::Right, is_home: false },

            // Row 3: Bottom alpha
            KeyDef { label: "N",   char_unshifted: Some('n'), char_shifted: Some('N'), finger: R_INDEX,  row: 3, col: 0, side: Hand::Right, is_home: false },
            KeyDef { label: "M",   char_unshifted: Some('m'), char_shifted: Some('M'), finger: R_INDEX,  row: 3, col: 1, side: Hand::Right, is_home: false },
            KeyDef { label: ",<",  char_unshifted: Some(','), char_shifted: Some('<'), finger: R_MIDDLE, row: 3, col: 2, side: Hand::Right, is_home: false },
            KeyDef { label: ".>",  char_unshifted: Some('.'), char_shifted: Some('>'), finger: R_RING,   row: 3, col: 3, side: Hand::Right, is_home: false },
            KeyDef { label: "/?",  char_unshifted: Some('/'), char_shifted: Some('?'), finger: R_PINKY,  row: 3, col: 4, side: Hand::Right, is_home: false },
            KeyDef { label: "Shf", char_unshifted: None,      char_shifted: None,      finger: R_PINKY,  row: 3, col: 5, side: Hand::Right, is_home: false },

            // Row 4: Bottom row
            KeyDef { label: "[{",  char_unshifted: Some('['), char_shifted: Some('{'), finger: R_RING,   row: 4, col: 0, side: Hand::Right, is_home: false },
            KeyDef { label: "]}",  char_unshifted: Some(']'), char_shifted: Some('}'), finger: R_PINKY,  row: 4, col: 1, side: Hand::Right, is_home: false },
            KeyDef { label: "Fn",  char_unshifted: None,      char_shifted: None,      finger: R_PINKY,  row: 4, col: 2, side: Hand::Right, is_home: false },

            // ═══════════════════════════════════════
            // RIGHT THUMB CLUSTER
            // ═══════════════════════════════════════
            KeyDef { label: "Win", char_unshifted: None,      char_shifted: None,      finger: R_THUMB,  row: 5, col: 0, side: Hand::Right, is_home: false },
            KeyDef { label: "Ctl", char_unshifted: None,      char_shifted: None,      finger: R_THUMB,  row: 5, col: 1, side: Hand::Right, is_home: false },
            KeyDef { label: "Ent", char_unshifted: None,      char_shifted: None,      finger: R_THUMB,  row: 6, col: 0, side: Hand::Right, is_home: false },
            KeyDef { label: "Spc", char_unshifted: Some(' '), char_shifted: Some(' '), finger: R_THUMB,  row: 6, col: 1, side: Hand::Right, is_home: false },
            KeyDef { label: "PgU", char_unshifted: None,      char_shifted: None,      finger: R_THUMB,  row: 7, col: 0, side: Hand::Right, is_home: false },
            KeyDef { label: "PgD", char_unshifted: None,      char_shifted: None,      finger: R_THUMB,  row: 7, col: 1, side: Hand::Right, is_home: false },
        ];

        let mut char_to_key = HashMap::new();
        for (i, key) in keys.iter().enumerate() {
            if let Some(c) = key.char_unshifted {
                char_to_key.insert(c, i);
            }
            if let Some(c) = key.char_shifted {
                char_to_key.entry(c).or_insert(i);
            }
        }

        Self { keys, char_to_key }
    }

    pub fn key_for_char(&self, c: char) -> Option<&KeyDef> {
        self.char_to_key.get(&c).map(|&i| &self.keys[i])
    }

    pub fn needs_shift(&self, c: char) -> bool {
        if let Some(&i) = self.char_to_key.get(&c) {
            self.keys[i].char_shifted == Some(c) && self.keys[i].char_unshifted != Some(c)
        } else {
            false
        }
    }

    pub fn finger_for_char(&self, c: char) -> Option<Finger> {
        self.key_for_char(c).map(|k| k.finger)
    }
}
