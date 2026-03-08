use super::curriculum::Stage;
use super::lesson::Lesson;
use super::typing::TypingSession;

/// Result of evaluating a typing session against lesson and stage requirements.
#[derive(Debug, Clone)]
pub struct ProgressionResult {
    pub passed: bool,
    pub wpm: f64,
    pub accuracy: f64,
    pub target_accuracy: f64,
    pub accuracy_passed: bool,
    /// WPM target for this stage (None if stage has no WPM gate).
    pub target_wpm: Option<f64>,
    pub wpm_passed: bool,
}

/// What should happen after a session result.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ProgressionAction {
    NextLesson(usize),
    Retry(usize),
    CurriculumComplete,
}

/// Evaluate a completed typing session against the lesson's requirements.
///
/// Early stages gate only on accuracy (the learner is just finding keys).
/// Later stages gate on both accuracy AND speed (building automaticity).
pub fn evaluate_session(session: &TypingSession, lesson: &Lesson) -> ProgressionResult {
    let accuracy = session.accuracy();
    let wpm = session.net_wpm();
    let accuracy_passed = accuracy >= lesson.target_accuracy;

    let (target_wpm, wpm_passed) = match lesson.stage {
        Stage::BigramFluency => (Some(40.0), wpm >= 40.0),
        Stage::WordChunking => (Some(60.0), wpm >= 60.0),
        Stage::RhythmAndFlow => (Some(80.0), wpm >= 80.0),
        _ => (None, true),
    };

    ProgressionResult {
        passed: accuracy_passed && wpm_passed,
        wpm,
        accuracy,
        target_accuracy: lesson.target_accuracy,
        accuracy_passed,
        target_wpm,
        wpm_passed,
    }
}

/// Determine the next action after a session result.
pub fn next_action(
    result: &ProgressionResult,
    current_lesson_id: usize,
    total_lessons: usize,
) -> ProgressionAction {
    if result.passed {
        if current_lesson_id + 1 < total_lessons {
            ProgressionAction::NextLesson(current_lesson_id + 1)
        } else {
            ProgressionAction::CurriculumComplete
        }
    } else {
        ProgressionAction::Retry(current_lesson_id)
    }
}
