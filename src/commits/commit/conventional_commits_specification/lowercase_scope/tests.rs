use rstest::rstest;

use super::*;

#[rstest(
    commit_message,
    // Lowercase scopes - should pass
    case("feat(auth): add login"),
    case("fix(api): handle errors"),
    case("docs(readme): update"),
    case("chore(deps): update packages"),
    case("test(user-service): add tests"),
    case("feat(api-client): add retry logic"),
    // No scope - should pass
    case("feat: add feature"),
    case("fix: bug fix"),
    case("docs: update docs"),
)]
fn test_lowercase_scope_passes(commit_message: &str) {
    let result = lint(commit_message);
    assert!(result.is_ok(), "Expected OK for: {}", commit_message);
}

#[rstest(
    commit_message,
    // Uppercase scopes - should fail
    case("feat(AUTH): add login"),
    case("fix(API): handle errors"),
    case("docs(README): update"),
    // Mixed case scopes - should fail
    case("feat(Auth): add login"),
    case("fix(ApiClient): handle errors"),
    case("docs(ReadMe): update"),
    case("chore(Deps): update packages"),
    // Scopes with uppercase letters in the middle
    case("feat(apiClient): add feature"),
    case("fix(userService): bug fix"),
)]
fn test_non_lowercase_scope_fails(commit_message: &str) {
    let result = lint(commit_message);
    assert_eq!(
        result,
        Err(LintingError::NonLowercaseScope),
        "Expected NonLowercaseScope error for: {}",
        commit_message
    );
}
