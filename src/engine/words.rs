use rand::seq::SliceRandom;
use rand::Rng;
use std::collections::HashSet;

/// Top English words sorted roughly by frequency.
const WORDS: &[&str] = &[
    "the", "be", "to", "of", "and", "a", "in", "that", "have", "i", "it", "for",
    "not", "on", "with", "he", "as", "you", "do", "at", "this", "but", "his",
    "by", "from", "they", "we", "her", "she", "or", "an", "will", "my", "one",
    "all", "would", "there", "their", "what", "so", "up", "out", "if", "about",
    "who", "get", "which", "go", "me", "when", "make", "can", "like", "time",
    "no", "just", "him", "know", "take", "people", "into", "year", "your",
    "good", "some", "could", "them", "see", "other", "than", "then", "now",
    "look", "only", "come", "its", "over", "think", "also", "back", "after",
    "use", "two", "how", "our", "work", "first", "well", "way", "even", "new",
    "want", "because", "any", "these", "give", "day", "most", "us", "great",
    "old", "tell", "ask", "find", "here", "thing", "many", "long", "big",
    "high", "such", "still", "own", "last", "life", "man", "call", "world",
    "very", "hand", "part", "live", "run", "place", "being", "under", "same",
    "right", "move", "try", "left", "late", "name", "turn", "large", "must",
    "home", "show", "end", "does", "point", "line", "state", "head", "need",
    "start", "far", "small", "down", "side", "been", "never", "each", "much",
    "might", "next", "more", "while", "house", "group", "case", "off", "set",
    "keep", "few", "light", "help", "near", "land", "child", "eye", "door",
    "open", "seem", "close", "night", "real", "play", "feel", "kind", "read",
    "stand", "face", "change", "water", "room", "stop", "city", "best",
    "body", "family", "school", "less", "idea", "study", "young", "hard",
    "table", "given", "three", "began", "often", "those", "told", "put",
    "done", "half", "plan", "full", "word", "fact", "hold", "hear", "form",
    "took", "book", "add", "sure", "area", "deep", "yet", "story", "age",
    "field", "mind", "able", "food", "talk", "job", "sort", "girl", "walk",
    "fish", "mark", "lead", "draw", "money", "note", "class", "got", "free",
    "fall", "bring", "look", "may", "said", "went", "made", "had", "did",
    "sat", "let", "got", "cut", "felt", "bit", "bad", "bed", "red", "dog",
    "top", "hot", "run", "sit", "hit", "bus", "cup", "sun", "ten", "yes",
    "air", "ago", "act", "art", "arm", "box", "car", "cry", "dry", "due",
    "ear", "eat", "egg", "fix", "fly", "fun", "gas", "hat", "ice", "ill",
    "key", "kid", "lay", "leg", "lip", "log", "lot", "map", "mix", "mud",
    "nor", "odd", "oil", "pay", "per", "pin", "pot", "pub", "raw", "rid",
    "row", "sad", "sea", "sir", "sky", "sum", "tea", "tie", "tip", "van",
    "via", "war", "wet", "win", "wood", "ship", "dark", "past", "deal",
    "grow", "wide", "cool", "fill", "safe", "sign", "mile", "ring", "rule",
    "join", "seem", "test", "rise", "flat", "firm", "soft", "drop", "rest",
    "roll", "shut", "step", "skin", "blow", "pick", "edge", "iron", "code",
    "pure", "push", "mass", "fine", "burn", "lift", "thus", "unit", "inch",
    "link", "gain", "folk", "band", "pool", "root", "tape", "hang", "gift",
    "rush", "deck", "fast", "plug", "mode", "tend", "tone", "bold", "slip",
    "snap", "spin", "peak", "span", "loop", "port", "dust", "palm",
];

/// Filter the word list to only contain words using the given set of characters.
pub fn words_for_chars(available: &HashSet<char>) -> Vec<&'static str> {
    WORDS
        .iter()
        .copied()
        .filter(|word| word.chars().all(|c| available.contains(&c)))
        .collect()
}

/// Generate exercise text from a word list.
pub fn generate_word_text(words: &[&str], target_len: usize, rng: &mut impl Rng) -> String {
    if words.is_empty() {
        return String::new();
    }
    let mut result = String::new();
    while result.len() < target_len {
        if !result.is_empty() {
            result.push(' ');
        }
        let word = words.choose(rng).unwrap();
        result.push_str(word);
    }
    result
}

/// Generate random character sequences from available chars (for early drills).
pub fn generate_char_drill(chars: &[char], target_len: usize, rng: &mut impl Rng) -> String {
    if chars.is_empty() {
        return String::new();
    }
    let mut result = String::new();
    let mut group_len = 0;
    while result.len() < target_len {
        if group_len >= 3 + rng.gen_range(0..3) {
            result.push(' ');
            group_len = 0;
        } else {
            result.push(*chars.choose(rng).unwrap());
            group_len += 1;
        }
    }
    result
}

/// Generate bigram-focused drill text.
pub fn generate_bigram_drill(
    chars: &[char],
    target_len: usize,
    rng: &mut impl Rng,
) -> String {
    if chars.len() < 2 {
        return generate_char_drill(chars, target_len, rng);
    }
    let mut result = String::new();
    let mut group_len = 0;
    while result.len() < target_len {
        if group_len >= 4 + rng.gen_range(0..3) {
            result.push(' ');
            group_len = 0;
        } else {
            let a = *chars.choose(rng).unwrap();
            let b = *chars.choose(rng).unwrap();
            result.push(a);
            result.push(b);
            group_len += 2;
        }
    }
    result
}

/// Generate column drill text for a single finger's column.
///
/// Trains vertical reaches on the columnar layout:
/// 1. Home → reach → home (basic reach training)
/// 2. Home → reach1 → home → reach2 (alternating reaches)
/// 3. Direct reach-to-reach (no home return, harder)
pub fn generate_column_drill(
    home: char,
    reaches: &[char],
    target_len: usize,
    rng: &mut impl Rng,
) -> String {
    if reaches.is_empty() {
        return generate_char_drill(&[home], target_len, rng);
    }

    let mut result = String::new();
    let mut phase = 0u8;

    while result.len() < target_len {
        if !result.is_empty() {
            result.push(' ');
        }

        match phase % 3 {
            // Phase 0: home-reach-home for a single reach key
            0 => {
                let reach = *reaches.choose(rng).unwrap();
                result.push(home);
                result.push(reach);
                result.push(home);
            }
            // Phase 1: home-reach1-home-reach2 (alternating two reaches)
            1 => {
                let r1 = *reaches.choose(rng).unwrap();
                let r2 = *reaches.choose(rng).unwrap();
                result.push(home);
                result.push(r1);
                result.push(home);
                result.push(r2);
                result.push(home);
            }
            // Phase 2: direct reach-to-reach traversal (no home return)
            2 => {
                let count = 3 + rng.gen_range(0..3);
                for i in 0..count {
                    if i == 0 {
                        result.push(home);
                    }
                    result.push(*reaches.choose(rng).unwrap());
                }
                result.push(home);
            }
            _ => unreachable!(),
        }
        phase = phase.wrapping_add(1);
    }

    result
}

/// Generate stagger-to-columnar transition drill text.
///
/// On a staggered keyboard, the bottom row is offset. On columnar, it's straight.
/// This generates text heavy in the key pairs that cause confusion during transition.
///
/// Each pair is (intended_key, commonly_confused_with). The drill emphasizes
/// the correct key and its column neighbors.
pub fn generate_confusion_drill(
    pairs: &[(char, char)],
    all_chars: &HashSet<char>,
    target_len: usize,
    rng: &mut impl Rng,
) -> String {
    if pairs.is_empty() {
        let chars: Vec<char> = all_chars.iter().copied().collect();
        return generate_bigram_drill(&chars, target_len, rng);
    }

    // Build a weighted char pool: confusion pair chars appear 3x more
    let mut pool: Vec<char> = all_chars.iter().copied().filter(|c| *c != ' ').collect();
    for (a, b) in pairs {
        if all_chars.contains(a) && all_chars.contains(b) {
            for _ in 0..3 {
                pool.push(*a);
                pool.push(*b);
            }
        }
    }

    let mut result = String::new();
    let mut group_len = 0;

    while result.len() < target_len {
        if group_len >= 3 + rng.gen_range(0..3) {
            result.push(' ');
            group_len = 0;
            continue;
        }

        // 60% chance: pick a confusion pair and drill it
        if rng.gen_range(0..10) < 6 && !pairs.is_empty() {
            let (a, b) = pairs.choose(rng).unwrap();
            if all_chars.contains(a) && all_chars.contains(b) {
                // Generate a short pattern with the pair
                match rng.gen_range(0..4) {
                    0 => {
                        result.push(*a);
                        result.push(*b);
                        group_len += 2;
                    }
                    1 => {
                        result.push(*b);
                        result.push(*a);
                        group_len += 2;
                    }
                    2 => {
                        result.push(*a);
                        result.push(*b);
                        result.push(*a);
                        group_len += 3;
                    }
                    _ => {
                        result.push(*b);
                        result.push(*a);
                        result.push(*b);
                        group_len += 3;
                    }
                }
            }
        } else {
            // Fill with a random char from the weighted pool
            result.push(*pool.choose(rng).unwrap());
            group_len += 1;
        }
    }

    result
}

/// Words that emphasize bottom-row keys (for stagger transition practice).
pub fn words_heavy_in(target_chars: &[char], available: &HashSet<char>) -> Vec<&'static str> {
    WORDS
        .iter()
        .copied()
        .filter(|word| {
            word.chars().all(|c| available.contains(&c))
                && word.chars().any(|c| target_chars.contains(&c))
        })
        .collect()
}

/// Generate text emphasizing specific target bigrams.
///
/// ~60% of character pairs come from the target bigrams, ~40% random filler.
/// Produces space-separated groups of 4-7 characters.
pub fn generate_targeted_bigram_drill(
    bigrams: &[(char, char)],
    chars: &HashSet<char>,
    target_len: usize,
    rng: &mut impl Rng,
) -> String {
    if bigrams.is_empty() {
        let chars: Vec<char> = chars.iter().copied().collect();
        return generate_bigram_drill(&chars, target_len, rng);
    }

    let filler: Vec<char> = chars.iter().copied().filter(|c| *c != ' ').collect();
    let mut result = String::new();
    let mut group_len = 0;

    while result.len() < target_len {
        if group_len >= 4 + rng.gen_range(0..4) {
            result.push(' ');
            group_len = 0;
            continue;
        }

        // 60% chance: emit a target bigram
        if rng.gen_range(0..10) < 6 {
            let (a, b) = bigrams[rng.gen_range(0..bigrams.len())];
            result.push(a);
            result.push(b);
            group_len += 2;
        } else if !filler.is_empty() {
            result.push(filler[rng.gen_range(0..filler.len())]);
            group_len += 1;
        }
    }

    result
}

/// Generate rhythm drill text: repeating short common word patterns.
///
/// Picks 2-3 short words and repeats them in a pattern. Changes the
/// pattern every ~40 characters to prevent pure memorization while
/// still allowing the typist to focus on even timing.
pub fn generate_rhythm_drill(target_len: usize, rng: &mut impl Rng) -> String {
    const SHORT_WORDS: &[&str] = &[
        "the", "and", "for", "are", "but", "not", "you", "all", "can", "had",
        "her", "was", "one", "our", "out", "has", "his", "how", "its", "may",
        "new", "now", "old", "see", "way", "who", "did", "get", "let", "say",
    ];

    let mut result = String::new();
    let mut chars_since_change = 0;

    // Pick initial pattern (2-3 words)
    let mut pattern: Vec<&str> = Vec::new();
    let pattern_len = 2 + rng.gen_range(0..2);
    for _ in 0..pattern_len {
        pattern.push(SHORT_WORDS[rng.gen_range(0..SHORT_WORDS.len())]);
    }
    let mut pattern_idx = 0;

    while result.len() < target_len {
        if !result.is_empty() {
            result.push(' ');
        }
        result.push_str(pattern[pattern_idx]);
        pattern_idx = (pattern_idx + 1) % pattern.len();
        chars_since_change += pattern[pattern_idx].len() + 1;

        // Change pattern every ~40 characters
        if chars_since_change >= 40 {
            // Swap one word in the pattern
            let swap_idx = rng.gen_range(0..pattern.len());
            pattern[swap_idx] = SHORT_WORDS[rng.gen_range(0..SHORT_WORDS.len())];
            pattern_idx = 0;
            chars_since_change = 0;
        }
    }

    result
}
