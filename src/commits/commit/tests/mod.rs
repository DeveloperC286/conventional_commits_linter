use rstest::rstest;
use rstest_reuse::{self, *};

use super::*;
use crate::commit_type::CommitType;

const DEFAULT_COMMIT_TYPE: &CommitType = &CommitType::Any;

const DEFAULT_COMMIT_TITLE_LENGTH: usize = 72;

const DEFAULT_LOWERCASE_SCOPE: bool = false;

#[template]
#[rstest(
    commit_message,
    // Normal variatiants.
    case("chore: 13.1.0"),
    case("fix: stop-parse was not being respected by commands (#1459)"),
    case("feat: zsh auto completion (#1292) "),
    case("fix: improve rpm release config (#64)\n\n* fix: improve rpm release config\r\n\r\nSigned-off-by: Carlos Alexandro Becker <caarlos0@gmail.com>\r\n\r\n* fix: template\r\n\r\nSigned-off-by: Carlos Alexandro Becker <caarlos0@gmail.com>\r\n"),
    case("feat: added support for \"rules\" debian package file (#49)\n\n* Added support for rules debian package file\r\n\r\n* Fixing typo in CONTRIBUTING.md\r\n\r\n* Runing 'make fmt' on project.\r\n"),
    case("ci: go mod tidy\n\nSigned-off-by: Carlos Alexandro Becker <caarlos0@gmail.com>\n"),
    // Breaking change variatiants.
    case("chore!: drop Node 6 support (#1461)"),
    case("refactor!: remove package.json-based parserConfiguration (#1460)"),
    case("fix!: calling parse multiple times now appropriately maintains state (#\n\n"),
    case("feat!: drop support for EOL Node 8 (#1686)"),
    // Scope variatiants.
    case("chore(major-release): release 17.7.0 (#2285)"),
    case("chore(deps): bump anchore/sbom-action from 0.13.3 to 0.13.4 (#637)"),
    case("feat(completion): zsh auto completion (#1292) "),
    case("fix(deps): Update os-locale to avoid security vulnerability (#1270)"),
    case("refactor(ts): ship yargs.d.ts (#1671)"),
    case("docs(readme): add Greenkeeper badge (#7)\n\nhttps://greenkeeper.io/"),
    case("feat(istanbul-reports): keyboard shortcuts on HTML report file (#265)\n\n* Keyboard shortcuts for low coverage in file listing view\r\n\r\n* Fix linting issues\r\n"),
    // TODO case("fix(#103): ensure the package is not included in itself  (#104)\n\n* fix(#103): ensure the package is not included in itself when using globs to match files\r\n\r\n* chore: switch strings.Contains to strings.HasSuffix\r\n"),
    // TODO case("fix(i18n): Japanese translation phrasing (#1619)\n\n"),
    // TODO case("fix(GO-2023-1621): update from go 1.20.1 to 1.20.2\n\nSigned-off-by: Carlos A Becker <caarlos0@users.noreply.github.com>\n"),
    case("fix(ecs-task-scheduler): changing min instances\n\n"),
    // TODO case("fix(win32): Detect files on different drive as outside project (#422)\n\nFixes #418"),
    // Scope variatiants https://gitlab.com/DeveloperC/conventional_commits_linter/-/issues/2.
    case("test(guest-agent): Add unit tests for spawn"),
    case("fix(guest-agent): Don't wait on failed spawns"),
    case("style(guest-agent): Fix typo"),
    case("feat(guest-agent): Run commands as the primary user"),
    // Breaking change and scope variatiants.
    case("chore(major-release)!: release 17.7.0 (#2285)"),
    case("feat(deps)!: yargs-parser now throws on invalid combinations of config (\n\n"),
    case("refactor(ts)!: ship yargs.d.ts (#1671)"),
    case("feat(guest-agent)!: run commands as the primary user"),
    case("fix(ecs-task-scheduler)!: changing min instances\n\n"),
)]
fn angular_type_conventional_commits(commit_message: &str) {}

#[apply(angular_type_conventional_commits)]
fn test_angular_type_conventional_commits_and_only_angular_type(commit_message: &str) {
    // Given
    let commit = Commit::from_commit_message(commit_message.to_string());
    let expected_linting_errors: Vec<LintingError> = vec![];

    // When
    let linting_errors = commit.lint(&CommitType::Angular, DEFAULT_COMMIT_TITLE_LENGTH, DEFAULT_LOWERCASE_SCOPE);

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
    let linting_errors = commit.lint(DEFAULT_COMMIT_TYPE, DEFAULT_COMMIT_TITLE_LENGTH, DEFAULT_LOWERCASE_SCOPE);

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
    case("samples: event.keyCode is deprecated, use new `.code` API (#2125)\n\nSome browser don't support `code` so I added a fallback for `keyCode`\r\n\r\nCo-authored-by: Benjamin E. Coe <bencoe@google.com>"),
    case("multiple: improved completion for choices\n\nfeat(completion): choices will now work for all possible aliases of an option and not just the default long option\r\nfix(completion): fix for completions that contain non-leading hyphens\r\nfix(completion): changed the check for option arguments to match options that begin with '-', instead of '--', to include short options"),
    case("bump: deps (#81)\n\nSigned-off-by: Carlos Alexandro Becker <caarlos0@gmail.com>"),
    // TODO case("i18n: Update zh_TW.json (#1976)\n\n"),
    case("meta: middleware improvements (#1852)\n\nfeat(middleware)!: global middleware now applied when no command is configured.\r\nfeat(middleware): async middleware can now be used before validation. "),
    case("doc: edit help example to align with actual output (#1271)\n\n"),
    case("tests: remove osx from CI, until Travis fixes OSX servers\n"),
    case("perfs: enhance contacts display and search (#2959)\n\n"),
    // Breaking change variatiants.
    case("feature!: incrementing API URL version (#1682)"),
    // Scope variatiants.
    case("deps(security): CVE-2021-3807\n\nUpdate string-width to 4.2.3"),
    case("doc(webpack): webpack example (#1436)\n\n* doc: weback example\r\n* doc(webpack): ignore dynamic module loading warnings"),
    case("deps(cve-dep-bump): CVE-2021-3807\n\nUpdate string-width to 4.2.3"),
    // Breaking change and scope variatiants.
    case("deps(security)!: CVE-2021-3807\n\nUpdate string-width to 4.2.3"),
    case("deps(cve-dep-bump)!: CVE-2021-3807\n\nUpdate string-width to 4.2.3"),
)]
fn non_angular_type_conventional_commits(commit_message: &str) {}

#[apply(non_angular_type_conventional_commits)]
fn test_non_angular_type_conventional_commits_and_only_angular_type(commit_message: &str) {
    // Given
    let commit = Commit::from_commit_message(commit_message.to_string());
    let expected_linting_errors = vec![LintingError::NonAngularType];

    // When
    let linting_errors = commit.lint(&CommitType::Angular, DEFAULT_COMMIT_TITLE_LENGTH, DEFAULT_LOWERCASE_SCOPE);

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
    let linting_errors = commit.lint(DEFAULT_COMMIT_TYPE, DEFAULT_COMMIT_TITLE_LENGTH, DEFAULT_LOWERCASE_SCOPE);

    // Then
    assert_eq!(
        expected_linting_errors, linting_errors,
        "\n\nFailed the assertion upon the commit message:\n{:?}\n\n",
        commit_message
    );
}

// All the various combinations of expected errors are tested through the generated tests.
// These tests cases are not exhaustive, but are used as a sanity check.
#[rstest(
    commit_message,
    case(" feat: adds support for async builder (#1888)\n\nFixes: #1042\r\nBREAKING CHANGE: providing an async builder will now cause yargs to return async result\r\nBREAKING CHANGE: .positional() now allowed at root level of yargs.\r\n"),
    case("Revert \"chore(deps): update dependency eslint to v7 (#1656)\" (#1673)\n\nThis reverts commit 1755aecc17311859a7cfa80807f997afb7883b7b."),
    case("Update advance.md: it's -> its (#1499)\n\nit's -> its"),
    case("Merge pull request #656 from dgrcode/translation/spanish\n\nSpanish translation for \"aliases\" and \"did you mean %s?\""),
)]
fn test_non_conventional_commits_fail_linting(commit_message: &str) {
    // Given
    let commit = Commit::from_commit_message(commit_message.to_string());

    // When
    let linting_errors = commit.lint(DEFAULT_COMMIT_TYPE, DEFAULT_COMMIT_TITLE_LENGTH, DEFAULT_LOWERCASE_SCOPE);

    // Then
    assert!(
        !linting_errors.is_empty(),
        "\n\nThe linting should have return errors for the commit message:\n{:?}\n\n",
        commit_message
    );
}

#[template]
#[rstest(
    commit_message,
    // Basic long commit messages
    case("fix: This is a very long commit message that exceeds the seventy-two character limit"),
    case("feat: Add new feature with a detailed description that goes over the limit"),
    case("docs: Update documentation with comprehensive examples and explanations that exceed length"),
    // Long commit messages with scopes
    case("fix(api): This is a very long commit message with scope that exceeds the seventy-two character limit"),
    case("feat(auth): Add comprehensive authentication system with detailed security features that exceed limit"),
    case("docs(readme): Update documentation with comprehensive examples and detailed explanations that exceed length"),
    case("chore(deps): Update all dependencies to latest versions with comprehensive security patches that exceed limit"),
    // Long commit messages with breaking changes
    case("fix!: This is a very long breaking change commit message that exceeds the seventy-two character limit"),
    case("feat!: Add new breaking feature with detailed description that goes over the character limit"),
    case("refactor!: Complete rewrite of core system with comprehensive changes that exceed the character limit"),
    // Long commit messages with scopes and breaking changes
    case("fix(api)!: This breaking change commit message with scope exceeds the seventy-two character limit"),
    case("feat(auth)!: Add comprehensive breaking authentication changes with detailed security that exceed limit"),
    // Long commit messages with bodies
    case("fix: This is a very long commit message that exceeds the seventy-two character limit\n\nThis commit includes a detailed body that explains the changes made to fix the issue."),
    case("feat: Add new feature with a detailed description that goes over the limit\n\nImplemented comprehensive feature set including:\n- Feature A\n- Feature B\n- Feature C"),
    case("docs: Update comprehensive documentation with examples that exceed the character limit\n\nUpdated all documentation files with new examples and improved explanations for better user experience."),
    // Long commit messages with scopes and bodies
    case("fix(api)!: Breaking API changes with very long title that exceeds character limit\n\nThis commit introduces breaking changes to the API:\n\n- Removed deprecated endpoints\n- Updated response format\n- Added new authentication requirements"),
    case("feat(ui): Comprehensive user interface overhaul that exceeds the character limit\n\nComplete redesign of the user interface including:\n- New component library\n- Improved accessibility\n- Mobile-first responsive design")
)]
fn commit_title_too_long_commits(commit_message: &str) {}

#[apply(commit_title_too_long_commits)]
fn test_commit_title_too_long(commit_message: &str) {
    // Given
    let commit = Commit::from_commit_message(commit_message.to_string());
    let expected_linting_errors: Vec<LintingError> = vec![LintingError::CommitTitleTooLong];

    // When
    let linting_errors = commit.lint(DEFAULT_COMMIT_TYPE, DEFAULT_COMMIT_TITLE_LENGTH, DEFAULT_LOWERCASE_SCOPE);

    // Then
    assert_eq!(
        expected_linting_errors, linting_errors,
        "\n\nFailed the assertion upon the commit message:\n{:?}\n\n",
        commit_message
    );
}

#[apply(commit_title_too_long_commits)]
fn test_max_commit_title_length_changeable(commit_message: &str) {
    // Given
    let commit = Commit::from_commit_message(commit_message.to_string());

    // When
    let linting_errors = commit.lint(DEFAULT_COMMIT_TYPE, 120, DEFAULT_LOWERCASE_SCOPE);

    // Then
    assert!(
        linting_errors.is_empty(),
        "\n\nThe linting should have returned no errors for the commit message:\n{:?}\n\n",
        commit_message
    );
}

#[apply(commit_title_too_long_commits)]
fn test_max_commit_title_length_disableable(commit_message: &str) {
    // Given
    let commit = Commit::from_commit_message(commit_message.to_string());

    // When
    let linting_errors = commit.lint(DEFAULT_COMMIT_TYPE, 0, DEFAULT_LOWERCASE_SCOPE);

    // Then
    assert!(
        linting_errors.is_empty(),
        "\n\nThe linting should have returned no errors for the commit message:\n{:?}\n\n",
        commit_message
    );
}

mod generated_tests;
