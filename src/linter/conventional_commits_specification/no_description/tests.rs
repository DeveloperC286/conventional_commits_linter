use rstest::rstest;

use super::*;

#[rstest(
    commit_message,
    case("fix:"),
    case("chore(release):\n\n"),
    case("chore(release):  \n\n"),
    case("lint: \n")
)]
fn test_no_description(commit_message: &str) {
    assert_eq!(lint(commit_message), Err(LintingError::NO_DESCRIPTION));
}

#[rstest(commit_message, case("chore!(release):  \n\n"))]
fn test_no_description_with_exclamation(commit_message: &str) {
    assert_eq!(lint(commit_message), Err(LintingError::NO_DESCRIPTION));
}

#[rstest(
    commit_message,
    case("  feat: "),
    case("\tchore:\n\n"),
    case(" major: \n")
)]
fn test_no_description_with_preceding_whitespace(commit_message: &str) {
    assert_eq!(lint(commit_message), Err(LintingError::NO_DESCRIPTION));
}

#[rstest(
    commit_message,
    case("chore(deps):\n\nCo-authored-by: Renovate Bot <bot@renovateapp.com>"),
    case("chore(deps):   \nCo-authored-by: Renovate Bot <bot@renovateapp.com>"),
    case("chore:    \n\nBumps [node-fetch](https://github.com/bitinn/node-fetch) from 2.6.0 to 2.6.1.\r\n- [Release notes](https://github.com/bitinn/node-fetch/releases)\r\n- [Changelog](https://github.com/node-fetch/node-fetch/blob/master/docs/CHANGELOG.md)\r\n- [Commits](https://github.com/bitinn/node-fetch/compare/v2.6.0...v2.6.1)\r\n\r\nSigned-off-by: dependabot[bot] <support@github.com>\r\n\r\nCo-authored-by: dependabot[bot] <49699333+dependabot[bot]@users.noreply.github.com>")
)]
fn test_no_description_with_body(commit_message: &str) {
    assert_eq!(lint(commit_message), Err(LintingError::NO_DESCRIPTION));
}

#[rstest(
    commit_message,
    case("feat: zsh auto completion (#1292) "),
    case("chore(release): 13.1.0"),
    case("fix!(deps): Update os-locale to avoid security vulnerability (#1270)"),
    case("fix: calling parse multiple times now appropriately maintains state (#\n\n")
)]
fn test_not_no_description(commit_message: &str) {
    assert!(lint(commit_message).is_ok());
}

#[rstest(
    commit_message,
    case("GetHandle() -> GetProcess().Handle()\n"),
    case("GetRenderProcessHost() has been removed for OOPI support, should use #include GetMainFrame()->GetProcess()\n")
)]
fn test_not_no_description_on_non_conventional_commit(commit_message: &str) {
    assert!(lint(commit_message).is_ok());
}
