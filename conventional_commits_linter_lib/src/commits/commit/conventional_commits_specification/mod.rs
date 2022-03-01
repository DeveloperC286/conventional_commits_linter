use regex::Regex;

use super::*;

pub(crate) mod empty_scope;
pub(crate) mod no_description_after_type_and_scope;
pub(crate) mod no_space_after_colon_preceding_type_and_scope;
pub(crate) mod preceding_whitespace;

pub(crate) fn lint(commit_message: &str) -> Result<(), LintingError> {
    lazy_static! {
        static ref CONVENTIONAL_COMMITS_REGEX: Regex = Regex::new(&format!(
            r"^{}{}{}: [^[[:space:]]]+",
            TYPE, OPTIONAL_SCOPE, OPTIONAL_EXCLAMATION,
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
