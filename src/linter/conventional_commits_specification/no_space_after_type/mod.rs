use crate::model::LintingError;
use regex::Regex;

pub fn lint(commit_message: &str) -> Result<(), LintingError> {
    lazy_static! {
        static ref NO_SPACE_AFTER_TYPE_REGEX: Regex =
            Regex::new(r"^([[:alpha:]])+(\([[:alpha:]]+\))?(!)?:($|[^ |:])").unwrap();
    }

    match NO_SPACE_AFTER_TYPE_REGEX.is_match(commit_message) {
        true => Err(LintingError::NO_SPACE_AFTER_TYPE),
        false => Ok(()),
    }
}
