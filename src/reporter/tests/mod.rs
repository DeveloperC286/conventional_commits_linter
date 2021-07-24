use rstest::rstest;

use super::*;

#[rstest(
    message,
    snapshot_name,
    case(
        "feat()!: yargs-parser now throws on invalid combinations of config\n\n",
        "test_stdin_pretty_print_case_1"
    ),
    case("Release 1.0.126", "test_stdin_pretty_print_case_2"),
    case(
        "Merge branch 'fix-ocsp-signer-check' into 'master'",
        "test_stdin_pretty_print_case_3"
    )
)]
fn test_stdin_pretty_print(message: &str, snapshot_name: &str) {
    // Given
    let commit = Commit {
        oid: git2::Oid::zero(),
        message: message.to_string(),
    };
    let linting_errors = crate::linter::lint_commit(&commit, false);

    // When
    let reporting = pretty_print_linting_error(None, message, &linting_errors);

    // Then
    insta::assert_snapshot!(snapshot_name, reporting);
}
