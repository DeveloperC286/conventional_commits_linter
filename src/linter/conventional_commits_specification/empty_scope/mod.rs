use regex::Regex;

use crate::model::LintingError;

pub fn lint(commit_message: &str) -> Result<(), LintingError> {
    lazy_static! {
        static ref EMPTY_SCOPE_REGEX: Regex = Regex::new(&format!(
            "{}{}{}{}{}:",
            *crate::linter::regex::OPTIONAL_PRECEDING_WHITESPACE,
            crate::linter::regex::TYPE,
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

#[cfg(test)]
mod tests;
