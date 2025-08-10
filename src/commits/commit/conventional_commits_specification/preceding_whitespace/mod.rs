use super::*;

static PRECEDING_WHITESPACE_REGEX: OnceLock<Regex> = OnceLock::new();

pub(crate) fn lint(commit_message: &str) -> Result<(), LintingError> {
    let regex = PRECEDING_WHITESPACE_REGEX
        .get_or_init(|| Regex::new(&format!("{PRECEDING_WHITESPACE}+")).unwrap());

    match regex.is_match(commit_message) {
        true => Err(LintingError::PrecedingWhitespace),
        false => Ok(()),
    }
}
