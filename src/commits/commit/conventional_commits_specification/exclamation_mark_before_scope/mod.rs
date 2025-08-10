use super::*;

static EXCLAMATION_MARK_BEFORE_SCOPE_REGEX: OnceLock<Regex> = OnceLock::new();

pub(crate) fn lint(commit_message: &str) -> Result<(), LintingError> {
    let regex = EXCLAMATION_MARK_BEFORE_SCOPE_REGEX.get_or_init(|| {
        Regex::new(&format!(
            "{OPTIONAL_PRECEDING_WHITESPACE}{TYPE}(!){EMPTY_SCOPE_OR_SCOPE}:",
        ))
        .unwrap()
    });

    match regex.is_match(commit_message) {
        true => Err(LintingError::ExclamationMarkBeforeScope),
        false => Ok(()),
    }
}
