use super::*;

pub(crate) fn lint(commit_message: &str) -> Result<(), LintingError> {
    lazy_static! {
        static ref PRECEDING_WHITESPACE_REGEX: Regex =
            Regex::new(&format!("{}+", PRECEDING_WHITESPACE)).unwrap();
    }

    match PRECEDING_WHITESPACE_REGEX.is_match(commit_message) {
        true => Err(LintingError::PrecedingWhitespace),
        false => Ok(()),
    }
}
