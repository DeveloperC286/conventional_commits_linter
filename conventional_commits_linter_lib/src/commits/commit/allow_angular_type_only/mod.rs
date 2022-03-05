use regex::Regex;

use super::*;

pub(crate) fn lint(commit_message: &str) -> Result<(), LintingError> {
    lazy_static! {
        static ref ANGULAR_TYPE_REGEX: Regex = Regex::new(&format!(
            r"(?i)^{}{ANGULAR_TYPE}{OPTIONAL_EXCLAMATION}{OPTIONAL_EMPTY_SCOPE_OR_SCOPE}{OPTIONAL_EXCLAMATION}:",
            *OPTIONAL_PRECEDING_WHITESPACE
        ))
        .unwrap();
    }

    match ANGULAR_TYPE_REGEX.is_match(commit_message) {
        true => Ok(()),
        false => Err(LintingError::NonAngularType),
    }
}
