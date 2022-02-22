use super::*;

pub(crate) fn lint(commit_message: &str) -> Result<(), LintingError> {
    lazy_static! {
        static ref EMPTY_SCOPE_REGEX: Regex = Regex::new(&format!(
            "{}{}{}{}{}:",
            *OPTIONAL_PRECEDING_WHITESPACE,
            TYPE,
            OPTIONAL_EXCLAMATION,
            EMPTY_SCOPE,
            OPTIONAL_EXCLAMATION
        ))
        .unwrap();
    }

    match EMPTY_SCOPE_REGEX.is_match(commit_message) {
        true => Err(LintingError::EmptyScope),
        false => Ok(()),
    }
}

#[cfg(test)]
mod tests;
