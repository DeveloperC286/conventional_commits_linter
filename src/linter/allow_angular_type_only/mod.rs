use regex::Regex;

use crate::model::LintingError;

pub fn lint(commit_message: &str) -> Result<(), LintingError> {
    lazy_static! {
        static ref ANGULAR_TYPE_REGEX: Regex = Regex::new(&format!(
            r"(?i)^{}(\([[:alpha:]]*\))?{}:",
            crate::linter::regex::ANGULAR_TYPE,
            crate::linter::regex::OPTIONAL_EXCLAMATION,
        ))
        .unwrap();
    }

    match ANGULAR_TYPE_REGEX.is_match(commit_message) {
        true => Ok(()),
        false => Err(LintingError::NON_ANGULAR_TYPE),
    }
}
