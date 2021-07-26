use rstest::rstest;

use super::*;

#[rstest(
    message,
    snapshot_name,
    case(
        "feat()!: yargs-parser now throws on invalid combinations of config\n\n",
        "test_pretty_print_case_1"
    ),
    case("Release 1.0.126", "test_pretty_print_case_2"),
    case(
        "Merge branch 'fix-ocsp-signer-check' into 'master'",
        "test_pretty_print_case_3"
    )
)]
fn test_pretty_print(message: &str, snapshot_name: &str) {
    // Given
    let commit = Commit {
        oid: git2::Oid::zero(),
        message: message.to_string(),
    };
    let linting_errors = crate::linter::lint_commit(&commit, false);

    // When
    let reporting = print(None, message, &linting_errors);

    // Then
    insta::assert_snapshot!(snapshot_name, reporting);
}

#[rstest(
    messages,
    snapshot_name,
    case(
        &["feat()!: yargs-parser now throws on invalid combinations of config\n\n", "doc(webpack):webpack example (#1436)"],
        "test_pretty_print_all_case_1"
    ),
    case(
        &["refactor(ts support)!: ship yargs.d.ts (#1671)", "feat!: drop support for EOL Node 8 (#1686)"],
        "test_pretty_print_all_case_2"
    ),
)]
fn test_pretty_print_all(messages: &[&str], snapshot_name: &str) {
    // Given
    let commits: Vec<Commit> = messages
        .iter()
        .enumerate()
        .map(|(index, message)| Commit {
            oid: git2::Oid::from_str(&*format!("{}", index)).unwrap(),
            message: message.to_string(),
        })
        .collect();
    let linting_errors = crate::linter::lint_commits(&commits, false);

    // When
    let reporting = print_all(&commits, &linting_errors);

    // Then
    insta::assert_snapshot!(snapshot_name, reporting);
}
