use super::*;

pub(crate) fn lint(commit_message: &str) -> Result<(), LintingError> {
    lazy_static! {
        static ref NO_SPACE_AFTER_TYPE_REGEX: Regex = Regex::new(&format!(
            r"{}($|[^ ])",
            *IGNORE_TYPE_AND_SCOPE_LINTING_ERRORS,
        ))
        .unwrap();
    }

    match NO_SPACE_AFTER_TYPE_REGEX.is_match(commit_message) {
        true => Err(LintingError::NoSpaceAfterColonPrecedingTypeAndScope),
        false => Ok(()),
    }
}
