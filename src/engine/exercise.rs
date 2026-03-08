use std::collections::HashSet;

use rand::Rng;

use super::words;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ExerciseType {
    /// Random characters from the available set
    CharDrill,
    /// Bigram-focused practice (random pairs)
    BigramDrill,
    /// Real words using only available characters
    Words,
    /// Column isolation: home key + vertical reaches
    ColumnDrill {
        home: char,
        reaches: Vec<char>,
    },
    /// Stagger-to-columnar transition: drill confused key pairs
    ConfusionPairs {
        pairs: Vec<(char, char)>,
    },
    /// Words weighted toward specific target characters
    WeightedWords {
        targets: Vec<char>,
    },
    /// Targeted bigram practice with specific high-frequency digraphs
    BigramTargeted {
        bigrams: Vec<(char, char)>,
    },
    /// Practice with a specific curated word set
    WordSet {
        words: Vec<&'static str>,
    },
    /// Rhythm drill: repeating patterns for consistent inter-keystroke timing
    RhythmDrill,
}

/// Generate exercise text for a given exercise type and character set.
pub fn generate_text(
    exercise_type: &ExerciseType,
    chars: &HashSet<char>,
    target_len: usize,
    rng: &mut impl Rng,
) -> String {
    match exercise_type {
        ExerciseType::CharDrill => {
            let chars: Vec<char> = chars.iter().copied().collect();
            words::generate_char_drill(&chars, target_len, rng)
        }
        ExerciseType::BigramDrill => {
            let chars: Vec<char> = chars.iter().copied().collect();
            words::generate_bigram_drill(&chars, target_len, rng)
        }
        ExerciseType::Words => {
            let available = words::words_for_chars(chars);
            if available.is_empty() {
                let chars: Vec<char> = chars.iter().copied().collect();
                words::generate_bigram_drill(&chars, target_len, rng)
            } else {
                words::generate_word_text(&available, target_len, rng)
            }
        }
        ExerciseType::ColumnDrill { home, reaches } => {
            words::generate_column_drill(*home, reaches, target_len, rng)
        }
        ExerciseType::ConfusionPairs { pairs } => {
            words::generate_confusion_drill(pairs, chars, target_len, rng)
        }
        ExerciseType::WeightedWords { targets } => {
            let weighted = words::words_heavy_in(targets, chars);
            if weighted.is_empty() {
                let chars: Vec<char> = chars.iter().copied().collect();
                words::generate_bigram_drill(&chars, target_len, rng)
            } else {
                words::generate_word_text(&weighted, target_len, rng)
            }
        }
        ExerciseType::BigramTargeted { bigrams } => {
            words::generate_targeted_bigram_drill(bigrams, chars, target_len, rng)
        }
        ExerciseType::WordSet { words: word_list } => {
            words::generate_word_text(word_list, target_len, rng)
        }
        ExerciseType::RhythmDrill => {
            words::generate_rhythm_drill(target_len, rng)
        }
    }
}
