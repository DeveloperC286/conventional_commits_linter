use crate::model::LintingError;
use regex::Regex;

pub fn lint(commit_message: &str) -> Result<(), LintingError> {
    lazy_static! {
        static ref NO_DESCRIPTION_REGEX: Regex = Regex::new(&format!(
            r"{}{}{}{}{}:([[:space:]]*($|\n\n))",
            *crate::linter::regex::OPTIONAL_PRECEDING_WHITESPACE,
            crate::linter::regex::TYPE,
            crate::linter::regex::OPTIONAL_EXCLAMATION,
            crate::linter::regex::OPTIONAL_EMPTY_SCOPE_OR_SCOPE,
            crate::linter::regex::OPTIONAL_EXCLAMATION
        ))
        .unwrap();
    }

    match NO_DESCRIPTION_REGEX.is_match(commit_message) {
        true => Err(LintingError::NO_DESCRIPTION),
        false => Ok(()),
    }
}
