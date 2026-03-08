use std::collections::HashMap;
use std::time::Instant;

use crate::keyboard::Finger;

/// A single recorded keystroke.
#[derive(Debug, Clone)]
pub struct Keystroke {
    pub expected: char,
    pub actual: char,
    pub correct: bool,
    pub timestamp: Instant,
}

/// Per-key statistics accumulated during a session.
#[derive(Debug, Clone, Default)]
pub struct KeyStats {
    pub correct: u32,
    pub errors: u32,
    pub total_ms: u64,
    pub count: u32,
}

impl KeyStats {
    pub fn accuracy(&self) -> f64 {
        let total = self.correct + self.errors;
        if total == 0 {
            1.0
        } else {
            self.correct as f64 / total as f64
        }
    }

    pub fn avg_ms(&self) -> f64 {
        if self.count == 0 {
            0.0
        } else {
            self.total_ms as f64 / self.count as f64
        }
    }
}

/// The live typing session — tracks everything that happens during an exercise.
pub struct TypingSession {
    pub text: Vec<char>,
    pub cursor: usize,
    pub keystrokes: Vec<Keystroke>,
    pub errors: Vec<bool>, // true if that position was an error at any point
    pub start_time: Option<Instant>,
    pub end_time: Option<Instant>,
    pub per_key_stats: HashMap<char, KeyStats>,
    pub per_bigram_stats: HashMap<(char, char), KeyStats>,
    last_key_time: Option<Instant>,
    last_char: Option<char>,
}

impl TypingSession {
    pub fn new(text: String) -> Self {
        let chars: Vec<char> = text.chars().collect();
        let len = chars.len();
        Self {
            text: chars,
            cursor: 0,
            keystrokes: Vec::new(),
            errors: vec![false; len],
            start_time: None,
            end_time: None,
            per_key_stats: HashMap::new(),
            per_bigram_stats: HashMap::new(),
            last_key_time: None,
            last_char: None,
        }
    }

    pub fn is_finished(&self) -> bool {
        self.cursor >= self.text.len()
    }

    pub fn current_char(&self) -> Option<char> {
        self.text.get(self.cursor).copied()
    }

    pub fn type_char(&mut self, c: char) {
        if self.is_finished() {
            return;
        }

        let now = Instant::now();

        // Start timer on first keystroke
        if self.start_time.is_none() {
            self.start_time = Some(now);
        }

        let expected = self.text[self.cursor];
        let correct = c == expected;

        // Record keystroke
        self.keystrokes.push(Keystroke {
            expected,
            actual: c,
            correct,
            timestamp: now,
        });

        // Update per-key stats
        let elapsed_ms = self
            .last_key_time
            .map(|t| now.duration_since(t).as_millis() as u64)
            .unwrap_or(0);

        let stats = self.per_key_stats.entry(expected).or_default();
        if correct {
            stats.correct += 1;
        } else {
            stats.errors += 1;
        }
        stats.total_ms += elapsed_ms;
        stats.count += 1;

        // Update bigram stats
        if let Some(prev) = self.last_char {
            let bigram_stats = self.per_bigram_stats.entry((prev, expected)).or_default();
            if correct {
                bigram_stats.correct += 1;
            } else {
                bigram_stats.errors += 1;
            }
            bigram_stats.total_ms += elapsed_ms;
            bigram_stats.count += 1;
        }

        if !correct {
            self.errors[self.cursor] = true;
        }

        // Advance cursor
        self.cursor += 1;
        self.last_key_time = Some(now);
        self.last_char = Some(expected);

        // Check if finished
        if self.is_finished() {
            self.end_time = Some(now);
        }
    }

    pub fn backspace(&mut self) {
        if self.cursor > 0 {
            self.cursor -= 1;
        }
    }

    /// Gross WPM: all typed characters / 5 / minutes
    pub fn gross_wpm(&self) -> f64 {
        let elapsed = self.elapsed_secs();
        if elapsed < 0.1 {
            return 0.0;
        }
        let chars_typed = self.cursor as f64;
        (chars_typed / 5.0) / (elapsed / 60.0)
    }

    /// Net WPM: (correct chars - errors) / 5 / minutes
    pub fn net_wpm(&self) -> f64 {
        let elapsed = self.elapsed_secs();
        if elapsed < 0.1 {
            return 0.0;
        }
        let error_count = self.errors.iter().take(self.cursor).filter(|&&e| e).count() as f64;
        let net_chars = (self.cursor as f64 - error_count).max(0.0);
        (net_chars / 5.0) / (elapsed / 60.0)
    }

    pub fn accuracy(&self) -> f64 {
        if self.cursor == 0 {
            return 1.0;
        }
        let error_count = self.errors.iter().take(self.cursor).filter(|&&e| e).count();
        1.0 - (error_count as f64 / self.cursor as f64)
    }

    pub fn elapsed_secs(&self) -> f64 {
        match (self.start_time, self.end_time) {
            (Some(start), Some(end)) => end.duration_since(start).as_secs_f64(),
            (Some(start), None) => Instant::now().duration_since(start).as_secs_f64(),
            _ => 0.0,
        }
    }

    pub fn progress(&self) -> f64 {
        if self.text.is_empty() {
            return 1.0;
        }
        self.cursor as f64 / self.text.len() as f64
    }

    /// Get the weakest keys by accuracy, for adaptive drilling.
    pub fn weakest_keys(&self, n: usize) -> Vec<(char, f64)> {
        let mut keys: Vec<(char, f64)> = self
            .per_key_stats
            .iter()
            .filter(|(_, stats)| stats.correct + stats.errors >= 3)
            .map(|(&c, stats)| (c, stats.accuracy()))
            .collect();
        keys.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap());
        keys.truncate(n);
        keys
    }

    /// Get the finger that produced the most errors.
    pub fn finger_accuracy(&self, layout: &crate::keyboard::Layout) -> HashMap<Finger, (u32, u32)> {
        let mut finger_stats: HashMap<Finger, (u32, u32)> = HashMap::new(); // (correct, errors)
        for (c, stats) in &self.per_key_stats {
            if let Some(finger) = layout.finger_for_char(*c) {
                let entry = finger_stats.entry(finger).or_default();
                entry.0 += stats.correct;
                entry.1 += stats.errors;
            }
        }
        finger_stats
    }
}
