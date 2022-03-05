use regex::Regex;

use super::*;

pub(super) mod empty_scope;
pub(super) mod no_description_after_type_and_scope;
pub(super) mod no_space_after_colon_preceding_type_and_scope;
pub(super) mod preceding_whitespace;

pub(super) fn lint(commit_message: &str) -> Result<(), LintingError> {
    lazy_static! {
        static ref CONVENTIONAL_COMMITS_REGEX: Regex = Regex::new(&format!(
            r"^{TYPE}{OPTIONAL_SCOPE}{OPTIONAL_EXCLAMATION}: [^[[:space:]]]+"
        ))
        .unwrap();
    }

    match CONVENTIONAL_COMMITS_REGEX.is_match(commit_message) {
        true => Ok(()),
        false => Err(LintingError::NonConventionalCommitsSpecification),
    }
}

#[cfg(test)]
mod tests;
