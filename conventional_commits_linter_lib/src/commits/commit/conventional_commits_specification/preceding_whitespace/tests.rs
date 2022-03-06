use rstest::rstest;

use super::*;

#[rstest(
    commit_message,
    case("\tfeat: zsh auto completion (#1292) "),
    case("  chore: 13.1.0\n\n"),
    case(" fix: Update os-locale to avoid security vulnerability (#1270)")
)]
fn test_preceding_whitespace(commit_message: &str) {
    assert_eq!(lint(commit_message), Err(LintingError::PrecedingWhitespace));
}

#[rstest(
    commit_message,
    case("GetHandle() -> GetProcess().Handle()\n"),
    case("GetRenderProcessHost() has been removed for OOPI support, should use #include GetMainFrame()->GetProcess()\n")
)]
fn test_not_preceding_whitespace_on_non_conventional_commit(commit_message: &str) {
    assert!(lint(commit_message).is_ok());
}
