use rstest::rstest;

use super::*;

#[rstest(
    commit_message,
    case(" fix(dependencies): upgrade yargs-parser to fix #1602  (#1603)\n\n"),
    case("\tfix: upgrade yargs-parser to fix #1602  (#1603)"),
    case(" chore: add id translation to #976 (#986)\n\n")
)]
fn test_preceding_formatting(commit_message: &str) {
    assert_eq!(
        lint(commit_message),
        Err(LintingError::PRECEDING_WHITESPACE)
    );
}
