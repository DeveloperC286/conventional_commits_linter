use super::*;

static ANGULAR_TYPE_REGEX: OnceLock<Regex> = OnceLock::new();

pub(super) fn lint(commit_message: &str) -> Result<(), LintingError> {
    let regex = ANGULAR_TYPE_REGEX.get_or_init(|| {
        Regex::new(&format!(
            r"(?i)^{OPTIONAL_PRECEDING_WHITESPACE}{ANGULAR_TYPE}{OPTIONAL_EXCLAMATION}{OPTIONAL_EMPTY_SCOPE_OR_SCOPE}{OPTIONAL_EXCLAMATION}:",
        ))
        .unwrap()
    });

    match regex.is_match(commit_message) {
        true => Ok(()),
        false => Err(LintingError::NonAngularType),
    }
}
