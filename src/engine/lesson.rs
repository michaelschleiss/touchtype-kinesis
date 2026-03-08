use std::collections::HashSet;

use rand::Rng;

use super::words;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ExerciseType {
    /// Random characters from the available set
    CharDrill,
    /// Bigram-focused practice
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
        match &self.exercise_type {
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
                    let chars: Vec<char> = self.chars.iter().copied().collect();
                    words::generate_bigram_drill(&chars, target_len, rng)
                } else {
                    words::generate_word_text(&available, target_len, rng)
                }
            }
            ExerciseType::ColumnDrill { home, reaches } => {
                words::generate_column_drill(*home, reaches, target_len, rng)
            }
            ExerciseType::ConfusionPairs { pairs } => {
                words::generate_confusion_drill(pairs, &self.chars, target_len, rng)
            }
            ExerciseType::WeightedWords { targets } => {
                let weighted = words::words_heavy_in(targets, &self.chars);
                if weighted.is_empty() {
                    let chars: Vec<char> = self.chars.iter().copied().collect();
                    words::generate_bigram_drill(&chars, target_len, rng)
                } else {
                    words::generate_word_text(&weighted, target_len, rng)
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

    let mut id = 0;
    let mut lessons = Vec::new();

    macro_rules! lesson {
        ($name:expr, $desc:expr, $chars:expr, $new:expr, $ex:expr, $acc:expr) => {{
            lessons.push(Lesson {
                id,
                name: $name,
                description: $desc,
                chars: $chars,
                new_chars: $new,
                exercise_type: $ex,
                target_accuracy: $acc,
            });
            id += 1;
        }};
    }

    // ═══════════════════════════════════════════════════════
    // Phase 1: Home Row Foundation
    // ═══════════════════════════════════════════════════════

    lesson!("Home Row Left", "Left hand home position: A S D F",
        home_left.clone(), vec!['a', 's', 'd', 'f'],
        ExerciseType::CharDrill, 0.90);

    lesson!("Home Row Right", "Right hand home position: J K L ;",
        home_right.clone(), vec!['j', 'k', 'l', ';'],
        ExerciseType::CharDrill, 0.90);

    lesson!("Home Row Combined", "Both hands: A S D F J K L ;",
        home_all.clone(), vec![],
        ExerciseType::BigramDrill, 0.90);

    lesson!("Index Reach: G H", "Index finger inner column: G and H",
        home_gh.clone(), vec!['g', 'h'],
        ExerciseType::BigramDrill, 0.90);

    lesson!("Vowels: E I", "Adding E (left middle) and I (right middle)",
        home_ei.clone(), vec!['e', 'i'],
        ExerciseType::BigramDrill, 0.90);

    lesson!("First Words", "Real words with home row + E, I",
        home_words.clone(), vec![' '],
        ExerciseType::Words, 0.92);

    // ═══════════════════════════════════════════════════════
    // Phase 2: Top Row
    // ═══════════════════════════════════════════════════════

    lesson!("Top Row Left: Q W R T", "Left hand top row (E already learned)",
        top_left.clone(), vec!['q', 'w', 'r', 't'],
        ExerciseType::BigramDrill, 0.88);

    lesson!("Top Row Right: Y U O P", "Right hand top row (I already learned)",
        top_all.clone(), vec!['y', 'u', 'o', 'p'],
        ExerciseType::Words, 0.88);

    lesson!("Top Row Words", "Words using home + top row",
        top_all.clone(), vec![],
        ExerciseType::Words, 0.92);

    // ═══════════════════════════════════════════════════════
    // Phase 3: Bottom Row
    // ═══════════════════════════════════════════════════════

    lesson!("Bottom Left: Z X C V B", "Left hand bottom row",
        bot_left.clone(), vec!['z', 'x', 'c', 'v', 'b'],
        ExerciseType::BigramDrill, 0.85);

    lesson!("Bottom Right: N M", "Right hand bottom row",
        bot_all.clone(), vec!['n', 'm'],
        ExerciseType::Words, 0.88);

    lesson!("Punctuation: , . / ?", "Common punctuation keys",
        punct.clone(), vec![',', '.', '/', '?'],
        ExerciseType::Words, 0.88);

    lesson!("All Letters", "Full alphabet word practice",
        all_alpha.clone(), vec!['\''],
        ExerciseType::Words, 0.92);

    // ═══════════════════════════════════════════════════════
    // Phase 4: Kinesis 360 Column Training
    //
    // The columnar layout means each finger owns a straight
    // vertical column. These drills train reaching up and
    // down within each column — the core motion of a
    // columnar keyboard.
    // ═══════════════════════════════════════════════════════

    lesson!("Column: Left Pinky", "Vertical reaches: A column (A Q Z)",
        chars_with(&all_alpha, " "), vec![],
        ExerciseType::ColumnDrill { home: 'a', reaches: vec!['q', 'z'] },
        0.88);

    lesson!("Column: Left Ring", "Vertical reaches: S column (S W X)",
        chars_with(&all_alpha, " "), vec![],
        ExerciseType::ColumnDrill { home: 's', reaches: vec!['w', 'x'] },
        0.88);

    lesson!("Column: Left Middle", "Vertical reaches: D column (D E C)",
        chars_with(&all_alpha, " "), vec![],
        ExerciseType::ColumnDrill { home: 'd', reaches: vec!['e', 'c'] },
        0.88);

    lesson!("Column: Left Index", "Vertical reaches: F/G columns (F R V T G B)",
        chars_with(&all_alpha, " "), vec![],
        ExerciseType::ColumnDrill { home: 'f', reaches: vec!['r', 'v', 't', 'g', 'b'] },
        0.85);

    lesson!("Column: Right Index", "Vertical reaches: J/H columns (J U M Y H N)",
        chars_with(&all_alpha, " "), vec![],
        ExerciseType::ColumnDrill { home: 'j', reaches: vec!['u', 'm', 'y', 'h', 'n'] },
        0.85);

    lesson!("Column: Right Middle", "Vertical reaches: K column (K I ,)",
        chars_with(&all_alpha, " ,"), vec![],
        ExerciseType::ColumnDrill { home: 'k', reaches: vec!['i', ','] },
        0.88);

    lesson!("Column: Right Ring", "Vertical reaches: L column (L O .)",
        chars_with(&all_alpha, " ."), vec![],
        ExerciseType::ColumnDrill { home: 'l', reaches: vec!['o', '.'] },
        0.88);

    lesson!("Column: Right Pinky", "Vertical reaches: ; column (; P /)",
        chars_with(&all_alpha, " /"), vec![],
        ExerciseType::ColumnDrill { home: ';', reaches: vec!['p', '/'] },
        0.85);

    // ═══════════════════════════════════════════════════════
    // Phase 5: Stagger-to-Columnar Transition
    //
    // On a staggered keyboard, bottom-row keys are offset.
    // Your muscle memory reaches diagonally. On the Kinesis
    // 360, the reach is straight down. These drills break
    // the old stagger habit.
    // ═══════════════════════════════════════════════════════

    // Left hand: stagger habit reaches down-and-left.
    // On columnar, C is directly under D (not between D and E).
    // X is directly under S (not between S and D).
    lesson!("Stagger Fix: Left Bottom", "Break diagonal habit for C X Z V B",
        chars_with(&all_alpha, " "), vec![],
        ExerciseType::ConfusionPairs {
            pairs: vec![
                ('d', 'c'), ('c', 'd'),  // c is straight below d
                ('s', 'x'), ('x', 's'),  // x is straight below s
                ('a', 'z'), ('z', 'a'),  // z is straight below a
                ('f', 'v'), ('v', 'f'),  // v is straight below f
                ('g', 'b'), ('b', 'g'),  // b is straight below g
            ],
        }, 0.88);

    // Right hand: stagger habit reaches down-and-right.
    // On columnar, N is directly under H (not between H and J).
    // M is directly under J (not between J and K).
    lesson!("Stagger Fix: Right Bottom", "Break diagonal habit for N M",
        chars_with(&all_alpha, " "), vec![],
        ExerciseType::ConfusionPairs {
            pairs: vec![
                ('h', 'n'), ('n', 'h'),  // n is straight below h
                ('j', 'm'), ('m', 'j'),  // m is straight below j
            ],
        }, 0.88);

    // Top row stagger: on staggered keyboards, top row is offset
    // in the opposite direction. On columnar, it's straight up.
    lesson!("Stagger Fix: Top Row", "Break diagonal habit for top row reaches",
        chars_with(&all_alpha, " "), vec![],
        ExerciseType::ConfusionPairs {
            pairs: vec![
                ('d', 'e'), ('e', 'd'),  // e is straight above d
                ('f', 'r'), ('r', 'f'),  // r is straight above f
                ('j', 'u'), ('u', 'j'),  // u is straight above j
                ('k', 'i'), ('i', 'k'),  // i is straight above k
                ('s', 'w'), ('w', 's'),  // w is straight above s
                ('l', 'o'), ('o', 'l'),  // o is straight above l
            ],
        }, 0.88);

    // Words that are heavy in bottom-row keys — real-world
    // practice for the stagger transition.
    lesson!("Stagger Words", "Words heavy in bottom-row keys",
        chars_with(&all_alpha, " "), vec![],
        ExerciseType::WeightedWords {
            targets: vec!['z', 'x', 'c', 'v', 'b', 'n', 'm'],
        }, 0.90);

    // ═══════════════════════════════════════════════════════
    // Phase 6: Numbers, Symbols, Speed
    // ═══════════════════════════════════════════════════════

    lesson!("Numbers", "Number row: 1 2 3 4 5 6 7 8 9 0",
        numbers.clone(), vec!['1', '2', '3', '4', '5', '6', '7', '8', '9', '0'],
        ExerciseType::CharDrill, 0.85);

    lesson!("Symbols", "Common symbols and shifted characters",
        symbols.clone(),
        vec!['!', '@', '#', '$', '%', '^', '&', '*', '(', ')', '-', '_', '=', '+'],
        ExerciseType::CharDrill, 0.80);

    lesson!("Speed Building", "Common words — build speed and consistency",
        all_alpha.clone(), vec![],
        ExerciseType::Words, 0.95);

    lessons
}
