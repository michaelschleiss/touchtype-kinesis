use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::PathBuf;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserProgress {
    pub highest_lesson: usize,
    pub total_sessions: u32,
    pub best_wpm: f64,
    pub best_accuracy: f64,
    pub per_key_accuracy: HashMap<char, f64>,
}

impl Default for UserProgress {
    fn default() -> Self {
        Self {
            highest_lesson: 0,
            total_sessions: 0,
            best_wpm: 0.0,
            best_accuracy: 0.0,
            per_key_accuracy: HashMap::new(),
        }
    }
}

impl UserProgress {
    fn path() -> Option<PathBuf> {
        dirs::config_dir().map(|d| d.join("kintype").join("progress.json"))
    }

    pub fn load() -> Self {
        Self::path()
            .and_then(|p| std::fs::read_to_string(p).ok())
            .and_then(|s| serde_json::from_str(&s).ok())
            .unwrap_or_default()
    }

    pub fn save(&self) {
        if let Some(path) = Self::path() {
            if let Some(parent) = path.parent() {
                let _ = std::fs::create_dir_all(parent);
            }
            if let Ok(json) = serde_json::to_string_pretty(self) {
                let _ = std::fs::write(path, json);
            }
        }
    }

    pub fn record_session(
        &mut self,
        lesson_id: usize,
        wpm: f64,
        accuracy: f64,
        passed: bool,
        per_key: &HashMap<char, crate::engine::typing::KeyStats>,
    ) {
        self.total_sessions += 1;

        if wpm > self.best_wpm {
            self.best_wpm = wpm;
        }
        if accuracy > self.best_accuracy {
            self.best_accuracy = accuracy;
        }

        if passed && lesson_id >= self.highest_lesson {
            self.highest_lesson = lesson_id + 1;
        }

        // Update per-key accuracy with exponential moving average
        for (ch, stats) in per_key {
            let new_acc = stats.accuracy();
            let entry = self.per_key_accuracy.entry(*ch).or_insert(new_acc);
            // EMA with alpha = 0.3 (weight recent sessions more)
            *entry = *entry * 0.7 + new_acc * 0.3;
        }

        self.save();
    }
}
