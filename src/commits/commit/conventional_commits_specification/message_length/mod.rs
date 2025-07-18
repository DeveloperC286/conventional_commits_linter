use super::*;

const MAX_SUBJECT_LENGTH: usize = 72;

pub fn lint(commit_message: &str) -> Result<(), LintingError> {
    let first_line = commit_message.lines().next().unwrap_or("");
    let subject_length = first_line.chars().count();
    
    if subject_length > MAX_SUBJECT_LENGTH {
        Err(LintingError::MessageTooLong)
    } else {
        Ok(())
    }
}