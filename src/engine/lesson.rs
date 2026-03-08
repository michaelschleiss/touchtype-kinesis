use std::collections::HashSet;

use rand::Rng;

use super::words;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ExerciseType {
    /// Random characters from the available set
    CharDrill,
    /// Bigram-focused practice
    BigramDrill,
    /// Real words using only available characters
    Words,
}

#[derive(Debug, Clone)]
pub struct Lesson {
    pub id: usize,
    pub name: &'static str,
    pub description: &'static str,
    /// The characters available in this lesson (cumulative)
    pub chars: HashSet<char>,
    /// The NEW characters introduced in this lesson
    pub new_chars: Vec<char>,
    pub exercise_type: ExerciseType,
    /// Target accuracy to "pass" (0.0 - 1.0)
    pub target_accuracy: f64,
}

impl Lesson {
    pub fn generate_text(&self, target_len: usize, rng: &mut impl Rng) -> String {
        match self.exercise_type {
            ExerciseType::CharDrill => {
                let chars: Vec<char> = self.chars.iter().copied().collect();
                words::generate_char_drill(&chars, target_len, rng)
            }
            ExerciseType::BigramDrill => {
                let chars: Vec<char> = self.chars.iter().copied().collect();
                words::generate_bigram_drill(&chars, target_len, rng)
            }
            ExerciseType::Words => {
                let available = words::words_for_chars(&self.chars);
                if available.is_empty() {
                    // Fall back to bigram drill if no words possible
                    let chars: Vec<char> = self.chars.iter().copied().collect();
                    words::generate_bigram_drill(&chars, target_len, rng)
                } else {
                    words::generate_word_text(&available, target_len, rng)
                }
            }
        }
    }
}

fn chars(s: &str) -> HashSet<char> {
    s.chars().collect()
}

fn chars_with(base: &HashSet<char>, extra: &str) -> HashSet<char> {
    let mut set = base.clone();
    for c in extra.chars() {
        set.insert(c);
    }
    set
}

pub fn all_lessons() -> Vec<Lesson> {
    // Build up character sets cumulatively
    let home_left = chars("asdf");
    let home_right = chars("jkl;");
    let home_all = chars_with(&home_left, "jkl;");
    let home_gh = chars_with(&home_all, "gh");
    let home_ei = chars_with(&home_gh, "ei");
    let home_words = chars_with(&home_ei, " ");
    let top_left = chars_with(&home_words, "qwrt");
    let top_all = chars_with(&top_left, "yuop");
    let bot_left = chars_with(&top_all, "zxcvb");
    let bot_all = chars_with(&bot_left, "nm");
    let punct = chars_with(&bot_all, ",./?");
    let all_alpha = chars_with(&punct, "'");
    let numbers = chars_with(&all_alpha, "1234567890");
    let symbols = chars_with(&numbers, "!@#$%^&*()-_=+[]{}\\|;:'\",.<>/?`~");

    vec![
        Lesson {
            id: 0,
            name: "Home Row Left",
            description: "Left hand home position: A S D F",
            chars: home_left.clone(),
            new_chars: vec!['a', 's', 'd', 'f'],
            exercise_type: ExerciseType::CharDrill,
            target_accuracy: 0.90,
        },
        Lesson {
            id: 1,
            name: "Home Row Right",
            description: "Right hand home position: J K L ;",
            chars: home_right.clone(),
            new_chars: vec!['j', 'k', 'l', ';'],
            exercise_type: ExerciseType::CharDrill,
            target_accuracy: 0.90,
        },
        Lesson {
            id: 2,
            name: "Home Row Combined",
            description: "Both hands: A S D F J K L ;",
            chars: home_all.clone(),
            new_chars: vec![],
            exercise_type: ExerciseType::BigramDrill,
            target_accuracy: 0.90,
        },
        Lesson {
            id: 3,
            name: "Index Reach: G H",
            description: "Index finger inner column: G and H",
            chars: home_gh.clone(),
            new_chars: vec!['g', 'h'],
            exercise_type: ExerciseType::BigramDrill,
            target_accuracy: 0.90,
        },
        Lesson {
            id: 4,
            name: "Vowels: E I",
            description: "Adding E (left middle) and I (right middle)",
            chars: home_ei.clone(),
            new_chars: vec!['e', 'i'],
            exercise_type: ExerciseType::BigramDrill,
            target_accuracy: 0.90,
        },
        Lesson {
            id: 5,
            name: "First Words",
            description: "Real words with home row + E, I",
            chars: home_words.clone(),
            new_chars: vec![' '],
            exercise_type: ExerciseType::Words,
            target_accuracy: 0.92,
        },
        Lesson {
            id: 6,
            name: "Top Row Left: Q W R T",
            description: "Left hand top row (E already learned)",
            chars: top_left.clone(),
            new_chars: vec!['q', 'w', 'r', 't'],
            exercise_type: ExerciseType::BigramDrill,
            target_accuracy: 0.88,
        },
        Lesson {
            id: 7,
            name: "Top Row Right: Y U O P",
            description: "Right hand top row (I already learned)",
            chars: top_all.clone(),
            new_chars: vec!['y', 'u', 'o', 'p'],
            exercise_type: ExerciseType::Words,
            target_accuracy: 0.88,
        },
        Lesson {
            id: 8,
            name: "Top Row Words",
            description: "Words using home + top row",
            chars: top_all.clone(),
            new_chars: vec![],
            exercise_type: ExerciseType::Words,
            target_accuracy: 0.92,
        },
        Lesson {
            id: 9,
            name: "Bottom Left: Z X C V B",
            description: "Left hand bottom row",
            chars: bot_left.clone(),
            new_chars: vec!['z', 'x', 'c', 'v', 'b'],
            exercise_type: ExerciseType::BigramDrill,
            target_accuracy: 0.85,
        },
        Lesson {
            id: 10,
            name: "Bottom Right: N M",
            description: "Right hand bottom row",
            chars: bot_all.clone(),
            new_chars: vec!['n', 'm'],
            exercise_type: ExerciseType::Words,
            target_accuracy: 0.88,
        },
        Lesson {
            id: 11,
            name: "Punctuation: , . / ?",
            description: "Common punctuation keys",
            chars: punct.clone(),
            new_chars: vec![',', '.', '/', '?'],
            exercise_type: ExerciseType::Words,
            target_accuracy: 0.88,
        },
        Lesson {
            id: 12,
            name: "All Letters",
            description: "Full alphabet word practice",
            chars: all_alpha.clone(),
            new_chars: vec!['\''],
            exercise_type: ExerciseType::Words,
            target_accuracy: 0.92,
        },
        Lesson {
            id: 13,
            name: "Numbers",
            description: "Number row: 1 2 3 4 5 6 7 8 9 0",
            chars: numbers.clone(),
            new_chars: vec!['1', '2', '3', '4', '5', '6', '7', '8', '9', '0'],
            exercise_type: ExerciseType::CharDrill,
            target_accuracy: 0.85,
        },
        Lesson {
            id: 14,
            name: "Symbols",
            description: "Common symbols and shifted characters",
            chars: symbols.clone(),
            new_chars: vec!['!', '@', '#', '$', '%', '^', '&', '*', '(', ')', '-', '_', '=', '+'],
            exercise_type: ExerciseType::CharDrill,
            target_accuracy: 0.80,
        },
        Lesson {
            id: 15,
            name: "Speed Building",
            description: "Common words — build speed and consistency",
            chars: all_alpha.clone(),
            new_chars: vec![],
            exercise_type: ExerciseType::Words,
            target_accuracy: 0.95,
        },
    ]
}
