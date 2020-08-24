use crate::model::LintingError;
use regex::Regex;

pub fn lint(commit_message: &str) -> Result<(), LintingError> {
    lazy_static! {
        static ref NO_DESCRIPTION_REGEX: Regex =
            Regex::new(r"^([[:alpha:]])+(\([[:alpha:]]+\))?(!)?:($|([[:space:]])+)").unwrap();
    }

    match NO_DESCRIPTION_REGEX.is_match(commit_message) {
        true => Err(LintingError::NO_DESCRIPTION),
        false => Ok(()),
    }
}
