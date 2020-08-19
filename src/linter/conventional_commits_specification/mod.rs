use crate::model::LintingError;
use regex::Regex;

pub mod empty_scope;

pub fn lint(commit_message: &str) -> Result<(), LintingError> {
    lazy_static! {
        static ref CONVENTIONAL_COMMITS_REGEX: Regex =
            Regex::new(r"^([[:alpha:]])+(\([[:alpha:]]+\))?(!)?: (.)+").unwrap();
    }

    match CONVENTIONAL_COMMITS_REGEX.is_match(commit_message) {
        true => Ok(()),
        false => Err(LintingError::NON_CONVENTIONAL_COMMITS_SPECIFICATION),
    }
}

#[cfg(test)]
mod tests;
