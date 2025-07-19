use super::*;

#[test]
fn test_lowercase_scope_passes() {
    let commit_message = "feat(auth): add login functionality";
    assert!(lint(commit_message).is_ok());
}

#[test]
fn test_lowercase_scope_with_dash_passes() {
    let commit_message = "fix(api-client): handle network errors";
    assert!(lint(commit_message).is_ok());
}

#[test]
fn test_uppercase_scope_fails() {
    let commit_message = "feat(AUTH): add login functionality";
    let result = lint(commit_message);
    assert!(result.is_err());
    assert_eq!(result.unwrap_err(), LintingError::NonLowercaseScope);
}

#[test]
fn test_mixed_case_scope_fails() {
    let commit_message = "fix(ApiClient): handle network errors";
    let result = lint(commit_message);
    assert!(result.is_err());
    assert_eq!(result.unwrap_err(), LintingError::NonLowercaseScope);
}

#[test]
fn test_no_scope_passes() {
    let commit_message = "feat: add login functionality";
    assert!(lint(commit_message).is_ok());
}

#[test]
fn test_scope_with_exclamation_lowercase_passes() {
    let commit_message = "feat(auth)!: breaking change to login";
    assert!(lint(commit_message).is_ok());
}

#[test]
fn test_scope_with_exclamation_uppercase_fails() {
    let commit_message = "feat(AUTH)!: breaking change to login";
    let result = lint(commit_message);
    assert!(result.is_err());
    assert_eq!(result.unwrap_err(), LintingError::NonLowercaseScope);
}
