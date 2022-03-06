use super::*;

pub(crate) fn lint(commit_message: &str) -> Result<(), LintingError> {
    lazy_static! {
        static ref EXCLAMATION_MARK_BEFORE_SCOPE_REGEX: Regex = Regex::new(&format!(
            "{}{TYPE}(!){EMPTY_SCOPE_OR_SCOPE}:",
            *OPTIONAL_PRECEDING_WHITESPACE
        ))
        .unwrap();
    }

    match EXCLAMATION_MARK_BEFORE_SCOPE_REGEX.is_match(commit_message) {
        true => Err(LintingError::ExclamationMarkBeforeScope),
        false => Ok(()),
    }
}
