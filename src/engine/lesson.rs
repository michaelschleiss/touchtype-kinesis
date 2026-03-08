use std::collections::HashSet;

use super::curriculum::Stage;
use super::exercise::ExerciseType;

#[derive(Debug, Clone)]
pub struct Lesson {
    pub id: usize,
    pub stage: Stage,
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
