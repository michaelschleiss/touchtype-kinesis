use std::collections::HashSet;

use super::exercise::ExerciseType;
use super::lesson::Lesson;

/// Cognitive science milestone stages for the typing curriculum.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum Stage {
    /// Declarative memory of key locations (0 -> ~20 WPM)
    KeyInternalization,
    /// Build correct muscle memory before speed (~20 -> 40 WPM)
    AccuracyFoundation,
    /// Break old staggered keyboard habits (Kinesis-specific)
    StaggerTransition,
    /// Chunk letter pairs into single motor units (~40 -> 60 WPM)
    BigramFluency,
    /// Entire words become motor programs (~60 -> 100 WPM)
    WordChunking,
    /// Consistent inter-keystroke timing (~80 -> 120+ WPM)
    RhythmAndFlow,
    /// Numbers and symbols
    NumbersAndSymbols,
}

/// Metadata for a curriculum stage.
#[derive(Debug, Clone)]
pub struct StageInfo {
    pub stage: Stage,
    pub name: &'static str,
    pub description: &'static str,
    pub cognitive_goal: &'static str,
    pub wpm_range: (u16, u16),
}

impl Stage {
    pub fn info(&self) -> StageInfo {
        match self {
            Stage::KeyInternalization => StageInfo {
                stage: *self,
                name: "Key Internalization",
                description: "Learn where every key lives on the columnar layout",
                cognitive_goal: "Build declarative memory of key locations",
                wpm_range: (0, 20),
            },
            Stage::AccuracyFoundation => StageInfo {
                stage: *self,
                name: "Accuracy Foundation",
                description: "Train correct columnar reaches with each finger",
                cognitive_goal: "Build correct muscle memory before speed",
                wpm_range: (20, 40),
            },
            Stage::StaggerTransition => StageInfo {
                stage: *self,
                name: "Stagger Transition",
                description: "Break diagonal habits from standard keyboards",
                cognitive_goal: "Rewire motor patterns for columnar layout",
                wpm_range: (20, 40),
            },
            Stage::BigramFluency => StageInfo {
                stage: *self,
                name: "Bigram Fluency",
                description: "Common letter pairs become single gestures",
                cognitive_goal: "Chunk digraphs into motor units",
                wpm_range: (40, 60),
            },
            Stage::WordChunking => StageInfo {
                stage: *self,
                name: "Word Chunking",
                description: "Whole words fire as single motor programs",
                cognitive_goal: "Automate word-level motor sequences",
                wpm_range: (60, 100),
            },
            Stage::RhythmAndFlow => StageInfo {
                stage: *self,
                name: "Rhythm & Flow",
                description: "Build speed with consistent keystroke timing",
                cognitive_goal: "Develop rhythmic automaticity",
                wpm_range: (80, 120),
            },
            Stage::NumbersAndSymbols => StageInfo {
                stage: *self,
                name: "Numbers & Symbols",
                description: "Extend to the full keyboard",
                cognitive_goal: "Internalize number row and shifted characters",
                wpm_range: (0, 0),
            },
        }
    }
}

/// The ordered list of all stages.
pub const ALL_STAGES: &[Stage] = &[
    Stage::KeyInternalization,
    Stage::AccuracyFoundation,
    Stage::StaggerTransition,
    Stage::BigramFluency,
    Stage::WordChunking,
    Stage::RhythmAndFlow,
    Stage::NumbersAndSymbols,
];

/// The full curriculum: stages and their lessons.
pub struct Curriculum {
    pub lessons: Vec<Lesson>,
}

impl Curriculum {
    pub fn new() -> Self {
        Self {
            lessons: build_lessons(),
        }
    }

    /// Lessons belonging to a given stage, in order.
    pub fn lessons_for_stage(&self, stage: Stage) -> Vec<&Lesson> {
        self.lessons.iter().filter(|l| l.stage == stage).collect()
    }

    /// The stage a given lesson_id belongs to.
    pub fn stage_for_lesson(&self, lesson_id: usize) -> Option<Stage> {
        self.lessons.get(lesson_id).map(|l| l.stage)
    }

    /// First lesson id in a given stage.
    pub fn first_lesson_in_stage(&self, stage: Stage) -> Option<usize> {
        self.lessons.iter().position(|l| l.stage == stage)
    }
}

// ─── Helpers ───

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

// ─── Lesson Definitions ───

fn build_lessons() -> Vec<Lesson> {
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
    let all_alpha_sp = chars_with(&all_alpha, " ");
    let numbers = chars_with(&all_alpha, "1234567890");
    let symbols = chars_with(&numbers, "!@#$%^&*()-_=+[]{}\\|;:'\",.<>/?`~");

    let mut id = 0;
    let mut lessons = Vec::new();

    macro_rules! lesson {
        ($stage:expr, $name:expr, $desc:expr, $chars:expr, $new:expr, $ex:expr, $acc:expr) => {{
            lessons.push(Lesson {
                id,
                stage: $stage,
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
    // Stage 1: Key Internalization
    // Learn where every key lives on the columnar layout.
    // Cognitive goal: declarative memory of key locations.
    // ═══════════════════════════════════════════════════════

    lesson!(Stage::KeyInternalization,
        "Home Row Left", "Left hand home position: A S D F",
        home_left.clone(), vec!['a', 's', 'd', 'f'],
        ExerciseType::CharDrill, 0.90);

    lesson!(Stage::KeyInternalization,
        "Home Row Right", "Right hand home position: J K L ;",
        home_right.clone(), vec!['j', 'k', 'l', ';'],
        ExerciseType::CharDrill, 0.90);

    lesson!(Stage::KeyInternalization,
        "Home Row Combined", "Both hands: A S D F J K L ;",
        home_all.clone(), vec![],
        ExerciseType::BigramDrill, 0.90);

    lesson!(Stage::KeyInternalization,
        "Index Reach: G H", "Index finger inner column: G and H",
        home_gh.clone(), vec!['g', 'h'],
        ExerciseType::BigramDrill, 0.90);

    lesson!(Stage::KeyInternalization,
        "Vowels: E I", "Adding E (left middle) and I (right middle)",
        home_ei.clone(), vec!['e', 'i'],
        ExerciseType::BigramDrill, 0.90);

    lesson!(Stage::KeyInternalization,
        "First Words", "Real words with home row + E, I",
        home_words.clone(), vec![' '],
        ExerciseType::Words, 0.92);

    lesson!(Stage::KeyInternalization,
        "Top Row Left: Q W R T", "Left hand top row (E already learned)",
        top_left.clone(), vec!['q', 'w', 'r', 't'],
        ExerciseType::BigramDrill, 0.88);

    lesson!(Stage::KeyInternalization,
        "Top Row Right: Y U O P", "Right hand top row (I already learned)",
        top_all.clone(), vec!['y', 'u', 'o', 'p'],
        ExerciseType::Words, 0.88);

    lesson!(Stage::KeyInternalization,
        "Top Row Words", "Words using home + top row",
        top_all.clone(), vec![],
        ExerciseType::Words, 0.92);

    lesson!(Stage::KeyInternalization,
        "Bottom Left: Z X C V B", "Left hand bottom row",
        bot_left.clone(), vec!['z', 'x', 'c', 'v', 'b'],
        ExerciseType::BigramDrill, 0.85);

    lesson!(Stage::KeyInternalization,
        "Bottom Right: N M", "Right hand bottom row",
        bot_all.clone(), vec!['n', 'm'],
        ExerciseType::Words, 0.88);

    lesson!(Stage::KeyInternalization,
        "Punctuation: , . / ?", "Common punctuation keys",
        punct.clone(), vec![',', '.', '/', '?'],
        ExerciseType::Words, 0.88);

    lesson!(Stage::KeyInternalization,
        "All Letters", "Full alphabet word practice",
        all_alpha.clone(), vec!['\''],
        ExerciseType::Words, 0.92);

    // ═══════════════════════════════════════════════════════
    // Stage 2: Accuracy Foundation
    // Train correct columnar reaches with each finger.
    // Cognitive goal: build correct muscle memory before speed.
    // ═══════════════════════════════════════════════════════

    lesson!(Stage::AccuracyFoundation,
        "Column: Left Pinky", "Vertical reaches: A column (A Q Z)",
        all_alpha_sp.clone(), vec![],
        ExerciseType::ColumnDrill { home: 'a', reaches: vec!['q', 'z'] },
        0.88);

    lesson!(Stage::AccuracyFoundation,
        "Column: Left Ring", "Vertical reaches: S column (S W X)",
        all_alpha_sp.clone(), vec![],
        ExerciseType::ColumnDrill { home: 's', reaches: vec!['w', 'x'] },
        0.88);

    lesson!(Stage::AccuracyFoundation,
        "Column: Left Middle", "Vertical reaches: D column (D E C)",
        all_alpha_sp.clone(), vec![],
        ExerciseType::ColumnDrill { home: 'd', reaches: vec!['e', 'c'] },
        0.88);

    lesson!(Stage::AccuracyFoundation,
        "Column: Left Index", "Vertical reaches: F/G columns (F R V T G B)",
        all_alpha_sp.clone(), vec![],
        ExerciseType::ColumnDrill { home: 'f', reaches: vec!['r', 'v', 't', 'g', 'b'] },
        0.85);

    lesson!(Stage::AccuracyFoundation,
        "Column: Right Index", "Vertical reaches: J/H columns (J U M Y H N)",
        all_alpha_sp.clone(), vec![],
        ExerciseType::ColumnDrill { home: 'j', reaches: vec!['u', 'm', 'y', 'h', 'n'] },
        0.85);

    lesson!(Stage::AccuracyFoundation,
        "Column: Right Middle", "Vertical reaches: K column (K I ,)",
        chars_with(&all_alpha, " ,"), vec![],
        ExerciseType::ColumnDrill { home: 'k', reaches: vec!['i', ','] },
        0.88);

    lesson!(Stage::AccuracyFoundation,
        "Column: Right Ring", "Vertical reaches: L column (L O .)",
        chars_with(&all_alpha, " ."), vec![],
        ExerciseType::ColumnDrill { home: 'l', reaches: vec!['o', '.'] },
        0.88);

    lesson!(Stage::AccuracyFoundation,
        "Column: Right Pinky", "Vertical reaches: ; column (; P /)",
        chars_with(&all_alpha, " /"), vec![],
        ExerciseType::ColumnDrill { home: ';', reaches: vec!['p', '/'] },
        0.85);

    // ═══════════════════════════════════════════════════════
    // Stage 3: Stagger Transition
    // Break diagonal habits from standard keyboards.
    // Cognitive goal: rewire motor patterns for columnar layout.
    // ═══════════════════════════════════════════════════════

    lesson!(Stage::StaggerTransition,
        "Stagger Fix: Left Bottom", "Break diagonal habit for C X Z V B",
        all_alpha_sp.clone(), vec![],
        ExerciseType::ConfusionPairs {
            pairs: vec![
                ('d', 'c'), ('c', 'd'),
                ('s', 'x'), ('x', 's'),
                ('a', 'z'), ('z', 'a'),
                ('f', 'v'), ('v', 'f'),
                ('g', 'b'), ('b', 'g'),
            ],
        }, 0.88);

    lesson!(Stage::StaggerTransition,
        "Stagger Fix: Right Bottom", "Break diagonal habit for N M",
        all_alpha_sp.clone(), vec![],
        ExerciseType::ConfusionPairs {
            pairs: vec![
                ('h', 'n'), ('n', 'h'),
                ('j', 'm'), ('m', 'j'),
            ],
        }, 0.88);

    lesson!(Stage::StaggerTransition,
        "Stagger Fix: Top Row", "Break diagonal habit for top row reaches",
        all_alpha_sp.clone(), vec![],
        ExerciseType::ConfusionPairs {
            pairs: vec![
                ('d', 'e'), ('e', 'd'),
                ('f', 'r'), ('r', 'f'),
                ('j', 'u'), ('u', 'j'),
                ('k', 'i'), ('i', 'k'),
                ('s', 'w'), ('w', 's'),
                ('l', 'o'), ('o', 'l'),
            ],
        }, 0.88);

    lesson!(Stage::StaggerTransition,
        "Stagger Words", "Words heavy in bottom-row keys",
        all_alpha_sp.clone(), vec![],
        ExerciseType::WeightedWords {
            targets: vec!['z', 'x', 'c', 'v', 'b', 'n', 'm'],
        }, 0.90);

    // ═══════════════════════════════════════════════════════
    // Stage 4: Bigram Fluency
    // Common letter pairs become single gestures.
    // Cognitive goal: chunk digraphs into motor units.
    // ═══════════════════════════════════════════════════════

    lesson!(Stage::BigramFluency,
        "Common Bigrams", "High-frequency pairs: TH, ER, IN, HE, AN, RE, ON",
        all_alpha_sp.clone(), vec![],
        ExerciseType::BigramTargeted {
            bigrams: vec![
                ('t', 'h'), ('h', 'e'), ('i', 'n'), ('e', 'r'),
                ('a', 'n'), ('r', 'e'), ('o', 'n'), ('e', 'n'),
                ('a', 't'), ('n', 'd'),
            ],
        }, 0.90);

    lesson!(Stage::BigramFluency,
        "Vowel Bigrams", "Vowel pairs: EA, OU, IO, AI, EI, IE, OO, EE",
        all_alpha_sp.clone(), vec![],
        ExerciseType::BigramTargeted {
            bigrams: vec![
                ('e', 'a'), ('o', 'u'), ('i', 'o'), ('a', 'i'),
                ('e', 'i'), ('i', 'e'), ('o', 'o'), ('e', 'e'),
                ('o', 'a'), ('u', 'e'),
            ],
        }, 0.90);

    lesson!(Stage::BigramFluency,
        "Bigram Words", "Words rich in common letter pairs",
        all_alpha_sp.clone(), vec![],
        ExerciseType::WeightedWords {
            targets: vec!['t', 'h', 'e', 'r', 'i', 'n', 'a'],
        }, 0.92);

    // ═══════════════════════════════════════════════════════
    // Stage 5: Word Chunking
    // Whole words fire as single motor programs.
    // Cognitive goal: automate word-level motor sequences.
    // ═══════════════════════════════════════════════════════

    lesson!(Stage::WordChunking,
        "Top 50 Words", "The 50 most common English words",
        all_alpha_sp.clone(), vec![],
        ExerciseType::WordSet {
            words: vec![
                "the", "be", "to", "of", "and", "a", "in", "that", "have", "i",
                "it", "for", "not", "on", "with", "he", "as", "you", "do", "at",
                "this", "but", "his", "by", "from", "they", "we", "her", "she", "or",
                "an", "will", "my", "one", "all", "would", "there", "their", "what", "so",
                "up", "out", "if", "about", "who", "get", "which", "go", "me", "when",
            ],
        }, 0.93);

    lesson!(Stage::WordChunking,
        "Top 100 Words", "Expanding to the 100 most common words",
        all_alpha_sp.clone(), vec![],
        ExerciseType::WordSet {
            words: vec![
                "the", "be", "to", "of", "and", "a", "in", "that", "have", "i",
                "it", "for", "not", "on", "with", "he", "as", "you", "do", "at",
                "this", "but", "his", "by", "from", "they", "we", "her", "she", "or",
                "an", "will", "my", "one", "all", "would", "there", "their", "what", "so",
                "up", "out", "if", "about", "who", "get", "which", "go", "me", "when",
                "make", "can", "like", "time", "no", "just", "him", "know", "take", "people",
                "into", "year", "your", "good", "some", "could", "them", "see", "other", "than",
                "then", "now", "look", "only", "come", "its", "over", "think", "also", "back",
                "after", "use", "two", "how", "our", "work", "first", "well", "way", "even",
                "new", "want", "because", "any", "these", "give", "day", "most", "us", "great",
            ],
        }, 0.93);

    lesson!(Stage::WordChunking,
        "Mixed Common Words", "Full vocabulary word practice",
        all_alpha_sp.clone(), vec![],
        ExerciseType::Words, 0.95);

    // ═══════════════════════════════════════════════════════
    // Stage 6: Rhythm & Flow
    // Build speed with consistent keystroke timing.
    // Cognitive goal: develop rhythmic automaticity.
    // ═══════════════════════════════════════════════════════

    lesson!(Stage::RhythmAndFlow,
        "Rhythm Drill", "Repeating patterns for consistent timing",
        all_alpha_sp.clone(), vec![],
        ExerciseType::RhythmDrill, 0.93);

    lesson!(Stage::RhythmAndFlow,
        "Speed Building", "Common words — build speed and consistency",
        all_alpha.clone(), vec![],
        ExerciseType::Words, 0.95);

    lesson!(Stage::RhythmAndFlow,
        "Flow Practice", "Longer passages for sustained rhythm",
        all_alpha_sp.clone(), vec![],
        ExerciseType::Words, 0.95);

    // ═══════════════════════════════════════════════════════
    // Stage 7: Numbers & Symbols
    // Extend to the full keyboard.
    // ═══════════════════════════════════════════════════════

    lesson!(Stage::NumbersAndSymbols,
        "Numbers", "Number row: 1 2 3 4 5 6 7 8 9 0",
        numbers.clone(), vec!['1', '2', '3', '4', '5', '6', '7', '8', '9', '0'],
        ExerciseType::CharDrill, 0.85);

    lesson!(Stage::NumbersAndSymbols,
        "Symbols", "Common symbols and shifted characters",
        symbols.clone(),
        vec!['!', '@', '#', '$', '%', '^', '&', '*', '(', ')', '-', '_', '=', '+'],
        ExerciseType::CharDrill, 0.80);

    lessons
}
