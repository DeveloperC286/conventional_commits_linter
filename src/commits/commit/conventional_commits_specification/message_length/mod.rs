use super::*;

pub fn lint(commit_message: &str, max_length: usize) -> Result<(), LintingError> {
    // If max_length is 0, skip the length check entirely
    if max_length == 0 {
        return Ok(());
    }

    let first_line = commit_message.lines().next().unwrap_or("");
    let subject_length = first_line.chars().count();

    if subject_length > max_length {
        Err(LintingError::MessageTooLong)
    } else {
        Ok(())
    }
}
