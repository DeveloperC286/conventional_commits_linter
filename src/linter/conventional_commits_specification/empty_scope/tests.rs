use rstest::rstest;

use super::*;

#[rstest(
    commit_message,
    case("feat(): zsh auto completion (#1292) "),
    case("chore(): 13.1.0\n\n"),
    case("fix(): Update os-locale to avoid security vulnerability (#1270)"),
    case("(): Update os-locale to avoid security vulnerability (#1270)\n\n"),
    case("chore()!: 13.1.0\n\n")
)]
fn test_empty_scope(commit_message: &str) {
    assert_eq!(lint(commit_message), Err(LintingError::EMPTY_SCOPE));
}

#[rstest(
    commit_message,
    case("fix(strict mode): report default command unknown arguments (#1626)")
)]
fn test_empty_scope_on_incorrect_scope(commit_message: &str) {
    assert!(lint(commit_message).is_ok());
}

#[rstest(
    commit_message,
    case("GetHandle() -> GetProcess().Handle()\n"),
    case("GetRenderProcessHost() has been removed for OOPI support, should use #include GetMainFrame()->GetProcess()\n"),
    case("GetHandle()! -> GetProcess().Handle()\n"),
    case("fix() Update os-locale to avoid security vulnerability (#1270)\n")
)]
fn test_empty_scope_on_non_conventional_commit(commit_message: &str) {
    assert!(lint(commit_message).is_ok());
}
