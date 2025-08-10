use super::*;

static NO_DESCRIPTION_REGEX: OnceLock<Regex> = OnceLock::new();

pub(crate) fn lint(commit_message: &str) -> Result<(), LintingError> {
    let regex = NO_DESCRIPTION_REGEX.get_or_init(|| {
        Regex::new(&format!(
            r"{}([[:space:]]*($|\n))",
            ignore_type_and_scope_linting_errors(),
        ))
        .unwrap()
    });

    match regex.is_match(commit_message) {
        true => Err(LintingError::NoDescriptionAfterTypeAndScope),
        false => Ok(()),
    }
}
