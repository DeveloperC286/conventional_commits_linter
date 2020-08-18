use crate::model::LintingError;
use regex::Regex;

pub fn lint(commit_message: &str) -> Result<(), LintingError> {
    lazy_static! {
        static ref ANGULAR_TYPE_REGEX: Regex = Regex::new(
            r"(?i)^(revert|build|ci|docs|feat|fix|perf|refactor|style|test)(\([[:alpha:]]*\))?(!)?:"
        )
        .unwrap();
    }

    match ANGULAR_TYPE_REGEX.is_match(commit_message) {
        true => Ok(()),
        false => Err(LintingError::NON_ANGULAR_TYPE),
    }
}

#[cfg(test)]
mod tests;
