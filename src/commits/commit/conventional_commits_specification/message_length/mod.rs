use super::*;

pub fn lint(commit_message: &str, max_length: Option<usize>) -> Result<(), LintingError> {
    // If max_length is None, skip the length check entirely
    let max_length = match max_length {
        Some(length) => length,
        None => return Ok(()),
    };

    let first_line = commit_message.lines().next().ok_or(LintingError::CommitTitleTooLong)?;
    let subject_length = first_line.chars().count();

    if subject_length > max_length {
        Err(LintingError::CommitTitleTooLong)
    } else {
        Ok(())
    }
}
