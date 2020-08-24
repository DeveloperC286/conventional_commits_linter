use crate::model::LintingError;
use regex::Regex;

pub fn lint(commit_message: &str) -> Result<(), LintingError> {
    lazy_static! {
        static ref EMPTY_SCOPE_REGEX: Regex = Regex::new(&format!(
            "{}*([[:alpha:]])*{}{}{}:",
            crate::linter::regex::PRECEDING_WHITESPACE,
            crate::linter::regex::OPTIONAL_EXCLAMATION,
            crate::linter::regex::EMPTY_SCOPE,
            crate::linter::regex::OPTIONAL_EXCLAMATION
        ))
        .unwrap();
    }

    match EMPTY_SCOPE_REGEX.is_match(commit_message) {
        true => Err(LintingError::EMPTY_SCOPE),
        false => Ok(()),
    }
}
