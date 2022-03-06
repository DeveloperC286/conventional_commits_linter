use rstest::rstest;
use rstest_reuse::{self, *};

use super::*;

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
    assert_eq!(linting_errors, vec![LintingError::NonAngularType]);
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

mod generated_tests;
