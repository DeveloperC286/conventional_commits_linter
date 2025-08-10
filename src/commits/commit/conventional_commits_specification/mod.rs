use super::*;

pub(super) mod empty_scope;
pub(super) mod exclamation_mark_before_scope;
pub mod message_length;
pub(super) mod no_description_after_type_and_scope;
pub(super) mod no_space_after_colon_preceding_type_and_scope;
pub(super) mod preceding_whitespace;

static CONVENTIONAL_COMMITS_REGEX: OnceLock<Regex> = OnceLock::new();

pub(super) fn lint(commit_message: &str) -> Result<(), LintingError> {
    let regex = CONVENTIONAL_COMMITS_REGEX.get_or_init(|| {
        Regex::new(&format!(
            r"^{TYPE}{OPTIONAL_SCOPE}{OPTIONAL_EXCLAMATION}: [^[[:space:]]]+"
        ))
        .unwrap()
    });

    match regex.is_match(commit_message) {
        true => Ok(()),
        false => Err(LintingError::NonConventionalCommitsSpecification),
    }
}
