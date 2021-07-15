use regex::Regex;

use crate::model::LintingError;

pub fn lint(commit_message: &str) -> Result<(), LintingError> {
    lazy_static! {
        static ref ANGULAR_TYPE_REGEX: Regex = Regex::new(&format!(
            r"(?i)^{}{}{}{}{}:",
            *crate::linter::regex::OPTIONAL_PRECEDING_WHITESPACE,
            crate::linter::regex::ANGULAR_TYPE,
            crate::linter::regex::OPTIONAL_EXCLAMATION,
            crate::linter::regex::OPTIONAL_EMPTY_SCOPE_OR_SCOPE,
            crate::linter::regex::OPTIONAL_EXCLAMATION,
        ))
        .unwrap();
    }

    match ANGULAR_TYPE_REGEX.is_match(commit_message) {
        true => Ok(()),
        false => Err(LintingError::NonAngularType),
    }
}
