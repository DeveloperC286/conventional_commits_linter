use super::*;

pub fn lint(commit_message: &str, max: Option<usize>) -> Result<(), LintingError> {
    if let Some(max) = max {
        let title = commit_message.lines().next().unwrap();
        let length = title.chars().count();

        if length > max {
            Err(LintingError::CommitTitleTooLong)
        } else {
            Ok(())
        }
    } else {
        Ok(())
    }
}
