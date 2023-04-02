use rstest::rstest;
use rstest_reuse::{self, *};

use super::*;

#[template]
#[rstest(
    commit_message,
    // Normal variatiants.
    case("test: add additional test for 1459"),
    case("fix: stop-parse was not being respected by commands (#1459)"),
    case("feat: zsh auto completion (#1292) "),
    case("fix: Update os-locale to avoid security vulnerability (#1270)"),
    case("fix: calling parse multiple times now appropriately maintains state (#\n\n"),
    case("feat: drop support for EOL Node 8 (#1686)"),
    // Breaking change variatiants.
    case("refactor!: remove package.json-based parserConfiguration (#1460)"),
    case("fix!: calling parse multiple times now appropriately maintains state (#\n\n"),
    case("feat!: drop support for EOL Node 8 (#1686)"),
    // Scope variatiants.
    case("feat(completion): zsh auto completion (#1292) "),
    case("fix(deps): Update os-locale to avoid security vulnerability (#1270)"),
    case("refactor(ts): ship yargs.d.ts (#1671)"),
    // Scope variatiants https://gitlab.com/DeveloperC/conventional_commits_linter/-/issues/2.
    case("test(guest-agent): Add unit tests for spawn"),
    case("fix(guest-agent): Don't wait on failed spawns"),
    case("style(guest-agent): Fix typo"),
    case("feat(guest-agent): Run commands as the primary user"),
    // Breaking change and scope variatiants.
    case("feat(deps)!: yargs-parser now throws on invalid combinations of config (\n\n"),
    case("refactor(ts)!: ship yargs.d.ts (#1671)"),
)]
fn angular_type_conventional_commits(commit_message: &str) {}

#[apply(angular_type_conventional_commits)]
fn test_angular_type_conventional_commits_and_only_angular_type(commit_message: &str) {
    // Given
    let commit = Commit::from_commit_message(commit_message.to_string());
    let expected_linting_errors: Vec<LintingError> = vec![];

    // When
    let linting_errors = commit.lint(true);

    // Then
    assert_eq!(
        expected_linting_errors, linting_errors,
        "\n\nFailed the assertion upon the commit message:\n{:?}\n\n",
        commit_message
    );
}

#[apply(angular_type_conventional_commits)]
fn test_angular_type_conventional_commits(commit_message: &str) {
    // Given
    let commit = Commit::from_commit_message(commit_message.to_string());
    let expected_linting_errors: Vec<LintingError> = vec![];

    // When
    let linting_errors = commit.lint(false);

    // Then
    assert_eq!(
        expected_linting_errors, linting_errors,
        "\n\nFailed the assertion upon the commit message:\n{:?}\n\n",
        commit_message
    );
}

#[template]
#[rstest(
    commit_message,
    // Normal variatiants.
    case("chore: 13.1.0"),
    case("samples: event.keyCode is deprecated, use new `.code` API (#2125)\n\nSome browser don't support `code` so I added a fallback for `keyCode`\r\n\r\nCo-authored-by: Benjamin E. Coe <bencoe@google.com>"),
    case("multiple: improved completion for choices\n\nfeat(completion): choices will now work for all possible aliases of an option and not just the default long option\r\nfix(completion): fix for completions that contain non-leading hyphens\r\nfix(completion): changed the check for option arguments to match options that begin with '-', instead of '--', to include short options"),
    case("bump: deps (#81)\n\nSigned-off-by: Carlos Alexandro Becker <caarlos0@gmail.com>"),
    // TODO case("i18n: Update zh_TW.json (#1976)\n\n"),
    case("meta: middleware improvements (#1852)\n\nfeat(middleware)!: global middleware now applied when no command is configured.\r\nfeat(middleware): async middleware can now be used before validation. "),
    case("doc: edit help example to align with actual output (#1271)\n\n"),
    case("tests: remove osx from CI, until Travis fixes OSX servers\n"),
    case("perfs: enhance contacts display and search (#2959)\n\n"),
    // Breaking change variatiants.
    case("chore!: drop Node 6 support (#1461)"),
    case("feature!: incrementing API URL version (#1682)"),
    // Scope variatiants.
    case("chore(major-release): release 17.7.0 (#2285)"),
    case("chore(deps): bump anchore/sbom-action from 0.13.3 to 0.13.4 (#637)"),
    case("deps(security): CVE-2021-3807\n\nUpdate string-width to 4.2.3"),
    case("doc(webpack): webpack example (#1436)\n\n* doc: weback example\r\n* doc(webpack): ignore dynamic module loading warnings"),
    // Breaking change and scope variatiants.
    case("chore(major-release)!: release 17.7.0 (#2285)"),
    case("deps(security)!: CVE-2021-3807\n\nUpdate string-width to 4.2.3"),
)]
fn non_angular_type_conventional_commits(commit_message: &str) {}

#[apply(non_angular_type_conventional_commits)]
fn test_non_angular_type_conventional_commits_and_only_angular_type(commit_message: &str) {
    // Given
    let commit = Commit::from_commit_message(commit_message.to_string());
    let expected_linting_errors = vec![LintingError::NonAngularType];

    // When
    let linting_errors = commit.lint(true);

    // Then
    assert_eq!(
        expected_linting_errors, linting_errors,
        "\n\nFailed the assertion upon the commit message:\n{:?}\n\n",
        commit_message
    );
}

#[apply(non_angular_type_conventional_commits)]
fn test_non_angular_type_conventional_commits(commit_message: &str) {
    // Given
    let commit = Commit::from_commit_message(commit_message.to_string());
    let expected_linting_errors: Vec<LintingError> = vec![];

    // When
    let linting_errors = commit.lint(false);

    // Then
    assert_eq!(
        expected_linting_errors, linting_errors,
        "\n\nFailed the assertion upon the commit message:\n{:?}\n\n",
        commit_message
    );
}

mod generated_tests;
