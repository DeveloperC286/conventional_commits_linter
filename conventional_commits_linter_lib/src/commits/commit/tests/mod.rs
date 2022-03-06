use insta::assert_debug_snapshot;
use rstest::rstest;
use rstest_reuse::{self, *};

use super::*;

#[macro_use]
mod macros;

#[template]
#[rstest(
    commit_message,
    case("feat(deps)!: yargs-parser now throws on invalid combinations of config (\n\n"),
    case("test: add additional test for 1459"),
    case("fix: stop-parse was not being respected by commands (#1459)"),
    case("refactor!: remove package.json-based parserConfiguration (#1460)"),
    case("feat: zsh auto completion (#1292) "),
    case("feat(completion): zsh auto completion (#1292) "),
    case("fix: Update os-locale to avoid security vulnerability (#1270)"),
    case("fix(deps): Update os-locale to avoid security vulnerability (#1270)"),
    case("fix!: calling parse multiple times now appropriately maintains state (#\n\n"),
    case("fix: calling parse multiple times now appropriately maintains state (#\n\n"),
    case("refactor(ts)!: ship yargs.d.ts (#1671)"),
    case("refactor(ts): ship yargs.d.ts (#1671)"),
    case("feat!: drop support for EOL Node 8 (#1686)"),
    case("feat: drop support for EOL Node 8 (#1686)")
)]
fn angular_type_conventional_commits(commit_message: &str) {}

#[apply(angular_type_conventional_commits)]
fn test_angular_type_conventional_commits_and_only_angular_type(commit_message: &str) {
    // Given
    let commit = Commit::from_commit_message(commit_message.to_string());

    // When
    let linting_errors = commit.lint(true);

    // Then
    assert!(linting_errors.is_empty());
}

#[apply(angular_type_conventional_commits)]
fn test_angular_type_conventional_commits(commit_message: &str) {
    // Given
    let commit = Commit::from_commit_message(commit_message.to_string());

    // When
    let linting_errors = commit.lint(false);

    // Then
    assert!(linting_errors.is_empty());
}

#[template]
#[rstest(
    commit_message,
    case("chore!: drop Node 6 support (#1461)"),
    case("doc(webpack): webpack example (#1436)"),
    case("chore(release): 14.2.0"),
    case("feature: support array of examples (#1682)"),
    case("chore(release): 13.1.0"),
    case("chore: 13.1.0")
)]
fn non_angular_type_conventional_commits(commit_message: &str) {}

#[apply(non_angular_type_conventional_commits)]
fn test_non_angular_type_conventional_commits_and_only_angular_type(commit_message: &str) {
    // Given
    let commit = Commit::from_commit_message(commit_message.to_string());

    // When
    let linting_errors = commit.lint(true);

    // Then
    assert_linting_errors_eq!(
        vec![LintingError::NonAngularType],
        linting_errors,
        commit_message
    );
}

#[apply(non_angular_type_conventional_commits)]
fn test_non_angular_type_conventional_commits(commit_message: &str) {
    // Given
    let commit = Commit::from_commit_message(commit_message.to_string());

    // When
    let linting_errors = commit.lint(false);

    // Then
    assert!(linting_errors.is_empty());
}

#[template]
#[rstest(
    commit_message,
    snapshot_name,
    case("!: drop Node 6 support (#1461)", "case_1"),
    case("(webpack): webpack example (#1436)", "case_2"),
    case("(release)!: 14.2.0", "case_3"),
    case(": support array of examples (#1682)", "case_4"),
    case("!(release): 13.1.0", "case_5"),
    case("  feature: adding zsh auto completion (#1292) ", "case_6"),
    case("\tcicd: releasing 13.1.0\n\n", "case_7"),
    case(
        "fix(): Update os-locale to avoid security vulnerability (#1270)",
        "case_8"
    ),
    case("chore()!: 13.1.0\n\n", "case_9"),
    case("feat!( ): zsh auto completion (#1292) ", "case_10"),
    case("  feat(  ): zsh auto completion (#1292) ", "case_11"),
    case("\tchore(): 13.1.0\n\n", "case_12"),
    case(" release!(): 13.1.0\n\n", "case_13"),
    case(" chore()!: releasing 13.1.0\n\n", "case_14"),
    case("fix:", "case_15"),
    case("chore(release):\n\n", "case_16"),
    case("chore(release):  \n\n", "case_17"),
    case("lint: \n", "case_18"),
    case("chore!(release):  \n\n", "case_19"),
    case("  feat: ", "case_20"),
    case("\tchore:\n\n", "case_21"),
    case(" major: \n", "case_22"),
    case("feat:zsh auto completion (#1292) ", "case_23"),
    case("chore(release):13.1.0", "case_24"),
    case(
        "fix!(deps):Update os-locale to avoid security vulnerability (#1270)",
        "case_25"
    ),
    case(
        "fix:calling parse multiple times now appropriately maintains state (#\n\n",
        "case_26"
    ),
    case(
        "fix(deps)!:Update os-locale to avoid security vulnerability (#1270)",
        "case_27"
    ),
    case(
        "  fix!(deps):Update os-locale to avoid security vulnerability (#1270)",
        "case_28"
    ),
    case(
        "\tfix:calling parse multiple times now appropriately maintains state (#\n\n",
        "case_29"
    ),
    case("\tfeat: zsh auto completion (#1292) ", "case_30")
)]
fn conventional_commits_with_errors(commit_message: &str, snapshot_name: &str) {}

#[apply(conventional_commits_with_errors)]
fn test_conventional_commits_with_errors(commit_message: &str, snapshot_name: &str) {
    // Given
    let commit = Commit::from_commit_message(commit_message.to_string());

    // When
    let linting_errors = commit.lint(false);

    // Then
    assert_debug_snapshot!(
        format!("test_conventional_commits_with_errors_{}", snapshot_name),
        linting_errors
    );
}

#[rstest(
    commit_message,
    case("chore(deps):\nCo-authored-by: Renovate Bot <bot@renovateapp.com>"),
    case("chore:\n\n")
)]
fn test_conventional_commits_no_description_and_no_space_after_type(commit_message: &str) {
    // Given
    let commit = Commit::from_commit_message(commit_message.to_string());

    // When
    let linting_errors = commit.lint(false);

    // Then
    assert_linting_errors_eq!(
        vec![
            LintingError::NonConventionalCommitsSpecification,
            LintingError::NoDescriptionAfterTypeAndScope,
            LintingError::NoSpaceAfterColonPrecedingTypeAndScope,
        ],
        linting_errors,
        commit_message
    );
}

#[rstest(
    commit_message,
    case("chore(deps):   \nCo-authored-by: Renovate Bot <bot@renovateapp.com>"),
    case("chore:    \n\nBumps [node-fetch](https://github.com/bitinn/node-fetch) from 2.6.0 to 2.6.1.\r\n- [Release notes](https://github.com/bitinn/node-fetch/releases)\r\n- [Changelog](https://github.com/node-fetch/node-fetch/blob/master/docs/CHANGELOG.md)\r\n- [Commits](https://github.com/bitinn/node-fetch/compare/v2.6.0...v2.6.1)\r\n\r\nSigned-off-by: dependabot[bot] <support@github.com>\r\n\r\nCo-authored-by: dependabot[bot] <49699333+dependabot[bot]@users.noreply.github.com>")
)]
fn test_conventional_commits_no_description(commit_message: &str) {
    // Given
    let commit = Commit::from_commit_message(commit_message.to_string());

    // When
    let linting_errors = commit.lint(false);

    // Then
    assert_linting_errors_eq!(
        vec![
            LintingError::NonConventionalCommitsSpecification,
            LintingError::NoDescriptionAfterTypeAndScope
        ],
        linting_errors,
        commit_message
    );
}

#[rstest(
    commit_message,
    case("GetHandle() -> GetProcess().Handle()\n"),
    case("GetRenderProcessHost() has been removed for OOPI support, should use #include GetMainFrame()->GetProcess()\n"),
)]
fn test_non_conventional_commits(commit_message: &str) {
    // Given
    let commit = Commit::from_commit_message(commit_message.to_string());

    // When
    let linting_errors = commit.lint(false);

    // Then
    assert_linting_errors_eq!(
        vec![LintingError::NonConventionalCommitsSpecification],
        linting_errors,
        commit_message
    );
}

mod generated_tests;
