use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Hand {
    Left,
    Right,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum FingerType {
    Pinky,
    Ring,
    Middle,
    Index,
    Thumb,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Finger {
    pub hand: Hand,
    pub finger: FingerType,
}

impl Finger {
    pub const fn new(hand: Hand, finger: FingerType) -> Self {
        Self { hand, finger }
    }

    pub fn color(&self) -> ratatui::style::Color {
        use ratatui::style::Color;
        match (self.hand, self.finger) {
            (Hand::Left, FingerType::Pinky) | (Hand::Right, FingerType::Pinky) => Color::Magenta,
            (Hand::Left, FingerType::Ring) | (Hand::Right, FingerType::Ring) => Color::Yellow,
            (Hand::Left, FingerType::Middle) | (Hand::Right, FingerType::Middle) => Color::Green,
            (Hand::Left, FingerType::Index) | (Hand::Right, FingerType::Index) => Color::Cyan,
            (Hand::Left, FingerType::Thumb) | (Hand::Right, FingerType::Thumb) => Color::Blue,
        }
    }

    pub fn label(&self) -> &'static str {
        match (self.hand, self.finger) {
            (Hand::Left, FingerType::Pinky) => "L Pinky",
            (Hand::Left, FingerType::Ring) => "L Ring",
            (Hand::Left, FingerType::Middle) => "L Mid",
            (Hand::Left, FingerType::Index) => "L Index",
            (Hand::Left, FingerType::Thumb) => "L Thumb",
            (Hand::Right, FingerType::Pinky) => "R Pinky",
            (Hand::Right, FingerType::Ring) => "R Ring",
            (Hand::Right, FingerType::Middle) => "R Mid",
            (Hand::Right, FingerType::Index) => "R Index",
            (Hand::Right, FingerType::Thumb) => "R Thumb",
        }
    }
}

pub const L_PINKY: Finger = Finger::new(Hand::Left, FingerType::Pinky);
pub const L_RING: Finger = Finger::new(Hand::Left, FingerType::Ring);
pub const L_MIDDLE: Finger = Finger::new(Hand::Left, FingerType::Middle);
pub const L_INDEX: Finger = Finger::new(Hand::Left, FingerType::Index);
pub const L_THUMB: Finger = Finger::new(Hand::Left, FingerType::Thumb);
pub const R_PINKY: Finger = Finger::new(Hand::Right, FingerType::Pinky);
pub const R_RING: Finger = Finger::new(Hand::Right, FingerType::Ring);
pub const R_MIDDLE: Finger = Finger::new(Hand::Right, FingerType::Middle);
pub const R_INDEX: Finger = Finger::new(Hand::Right, FingerType::Index);
pub const R_THUMB: Finger = Finger::new(Hand::Right, FingerType::Thumb);
