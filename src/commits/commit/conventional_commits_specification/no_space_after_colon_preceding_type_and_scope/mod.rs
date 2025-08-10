use super::*;

static NO_SPACE_AFTER_TYPE_REGEX: OnceLock<Regex> = OnceLock::new();

pub(crate) fn lint(commit_message: &str) -> Result<(), LintingError> {
    let regex = NO_SPACE_AFTER_TYPE_REGEX.get_or_init(|| {
        Regex::new(&format!(
            r"{}($|[^ ])",
            ignore_type_and_scope_linting_errors(),
        ))
        .unwrap()
    });

    match regex.is_match(commit_message) {
        true => Err(LintingError::NoSpaceAfterColonPrecedingTypeAndScope),
        false => Ok(()),
    }
}
