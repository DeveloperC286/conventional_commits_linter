use regex::Regex;

use crate::LintingError;

pub(crate) fn lint(commit_message: &str) -> Result<(), LintingError> {
    lazy_static! {
        static ref NO_DESCRIPTION_REGEX: Regex = Regex::new(&format!(
            r"{}([[:space:]]*($|\n))",
            *crate::linter::regex::IGNORE_TYPE_AND_SCOPE_LINTING_ERRORS,
        ))
        .unwrap();
    }

    match NO_DESCRIPTION_REGEX.is_match(commit_message) {
        true => Err(LintingError::NoDescriptionAfterTypeAndScope),
        false => Ok(()),
    }
}

#[cfg(test)]
mod tests;