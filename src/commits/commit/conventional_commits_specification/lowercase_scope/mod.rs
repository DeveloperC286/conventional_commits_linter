use super::*;

pub(crate) fn lint(commit_message: &str) -> Result<(), LintingError> {
    lazy_static! {
        static ref SCOPE_REGEX: Regex = Regex::new(r"\(([^)]+)\)").unwrap();
    }

    match SCOPE_REGEX.captures(commit_message) {
        Some(captures) => {
            if let Some(scope_match) = captures.get(1) {
                let scope = scope_match.as_str();
                if scope != scope.to_lowercase() {
                    return Err(LintingError::NonLowercaseScope);
                }
            }
            Ok(())
        }
        None => Ok(()), // No scope present, that's fine
    }
}

#[cfg(test)]
mod tests;
