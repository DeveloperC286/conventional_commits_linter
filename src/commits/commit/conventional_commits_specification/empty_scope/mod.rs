use super::*;

static EMPTY_SCOPE_REGEX: OnceLock<Regex> = OnceLock::new();

pub(crate) fn lint(commit_message: &str) -> Result<(), LintingError> {
    let regex = EMPTY_SCOPE_REGEX.get_or_init(|| {
        Regex::new(&format!(
            "{OPTIONAL_PRECEDING_WHITESPACE}{TYPE}{OPTIONAL_EXCLAMATION}{EMPTY_SCOPE}{OPTIONAL_EXCLAMATION}:",
        ))
        .unwrap()
    });

    match regex.is_match(commit_message) {
        true => Err(LintingError::EmptyScope),
        false => Ok(()),
    }
}
