use super::*;

static SCOPE_EXTRACTION_REGEX: OnceLock<Regex> = OnceLock::new();

pub(crate) fn lint(commit_message: &str) -> Result<(), LintingError> {
    let regex = SCOPE_EXTRACTION_REGEX.get_or_init(|| {
        Regex::new(&format!(
            r"^{OPTIONAL_PRECEDING_WHITESPACE}{TYPE}{OPTIONAL_EXCLAMATION}\(([^\)]+)\){OPTIONAL_EXCLAMATION}:",
        ))
        .unwrap()
    });

    if let Some(captures) = regex.captures(commit_message) {
        if let Some(scope) = captures.get(1) {
            let scope_text = scope.as_str();
            // Check if the scope contains any uppercase letters
            if scope_text.chars().any(|c| c.is_uppercase()) {
                return Err(LintingError::NonLowercaseScope);
            }
        }
    }

    Ok(())
}

#[cfg(test)]
mod tests;
