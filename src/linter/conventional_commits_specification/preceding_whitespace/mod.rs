use crate::model::LintingError;
use regex::Regex;

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
