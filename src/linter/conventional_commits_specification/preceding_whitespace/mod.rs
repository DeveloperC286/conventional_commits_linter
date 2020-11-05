use regex::Regex;

use crate::model::LintingError;

pub fn lint(commit_message: &str) -> Result<(), LintingError> {
    lazy_static! {
        static ref PRECEDING_WHITESPACE_REGEX: Regex =
            Regex::new(&format!("{}+", crate::linter::regex::PRECEDING_WHITESPACE)).unwrap();
    }

    match PRECEDING_WHITESPACE_REGEX.is_match(commit_message) {
        true => Err(LintingError::PRECEDING_WHITESPACE),
        false => Ok(()),
    }
}
