use rstest::rstest;

use super::*;

#[rstest(
    commit_message,
    case("feat(): zsh auto completion (#1292) "),
    case("chore(): 13.1.0\n\n"),
    case("fix(): Update os-locale to avoid security vulnerability (#1270)"),
    case("(): Update os-locale to avoid security vulnerability (#1270)\n\n"),
    case("fix() Update os-locale to avoid security vulnerability (#1270)\n")
)]
fn test_empty_scope(commit_message: &str) {
    assert_eq!(lint(commit_message), Err(LintingError::EMPTY_SCOPE));
}

#[rstest(
    commit_message,
    case("fix(strict mode): report default command unknown arguments (#1626)")
)]
fn test_empty_scope_ignores_incorrect_scope(commit_message: &str) {
    assert!(lint(commit_message).is_ok());
}
