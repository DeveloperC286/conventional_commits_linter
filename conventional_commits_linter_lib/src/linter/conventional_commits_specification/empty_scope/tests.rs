use rstest::rstest;

use super::*;

#[rstest(
    commit_message,
    case("feat(  ): zsh auto completion (#1292) "),
    case("chore(\t): 13.1.0\n\n"),
    case("fix(): Update os-locale to avoid security vulnerability (#1270)")
)]
fn test_empty_scope(commit_message: &str) {
    assert_eq!(lint(commit_message), Err(LintingError::EmptyScope));
}

#[rstest(
    commit_message,
    case("chore()!: 13.1.0\n\n"),
    case("feat!( ): zsh auto completion (#1292) ")
)]
fn test_empty_scope_with_exclamation(commit_message: &str) {
    assert_eq!(lint(commit_message), Err(LintingError::EmptyScope));
}

#[rstest(
    commit_message,
    case("  feat(  ): zsh auto completion (#1292) "),
    case("\tchore(): 13.1.0\n\n")
)]
fn test_empty_scope_with_preceding_whitespace(commit_message: &str) {
    assert_eq!(lint(commit_message), Err(LintingError::EmptyScope));
}

#[rstest(
    commit_message,
    case("feat(completion): zsh auto completion (#1292) "),
    case("chore(release): 13.1.0"),
    case("fix(deps): Update os-locale to avoid security vulnerability (#1270)"),
    case("fix!: calling parse multiple times now appropriately maintains state (#\n\n")
)]
fn test_not_empty_scope(commit_message: &str) {
    assert!(lint(commit_message).is_ok());
}

#[rstest(
    commit_message,
    case("GetHandle() -> GetProcess().Handle()\n"),
    case("GetRenderProcessHost() has been removed for OOPI support, should use #include GetMainFrame()->GetProcess()\n")
)]
fn test_not_empty_scope_on_non_conventional_commit(commit_message: &str) {
    assert!(lint(commit_message).is_ok());
}