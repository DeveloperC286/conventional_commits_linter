use regex::Regex;

use crate::model::LintingError;

pub mod empty_scope;
pub mod no_description;
pub mod no_space_after_colon_preceding_type_and_scope;
pub mod preceding_whitespace;

pub fn lint(commit_message: &str) -> Result<(), LintingError> {
    lazy_static! {
        static ref CONVENTIONAL_COMMITS_REGEX: Regex = Regex::new(&format!(
            r"^{}{}{}: [^[[:space:]]]+",
            crate::linter::regex::TYPE,
            crate::linter::regex::OPTIONAL_SCOPE,
            crate::linter::regex::OPTIONAL_EXCLAMATION,
        ))
        .unwrap();
    }

    match CONVENTIONAL_COMMITS_REGEX.is_match(commit_message) {
        true => Ok(()),
        false => Err(LintingError::NonConventionalCommitsSpecification),
    }
}
