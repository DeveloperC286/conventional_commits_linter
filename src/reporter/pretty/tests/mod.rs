use rstest::rstest;

use super::*;

#[rstest(
    commit_message,
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
fn test_pretty_print(commit_message: &str, snapshot_name: &str) {
    // Given
    let linting_errors = crate::linter::lint_commit_message(commit_message, false);

    // When
    let reporting = print(None, commit_message, &linting_errors);

    // Then
    insta::assert_snapshot!(snapshot_name, reporting);
}

#[rstest(
    commit_messages,
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
fn test_pretty_print_all(commit_messages: &[&str], snapshot_name: &str) {
    // Given
    let commits: Vec<Commit> = commit_messages
        .iter()
        .enumerate()
        .map(|(index, commit_message)| Commit {
            oid: git2::Oid::from_str(&*format!("{}", index)).unwrap(),
            message: commit_message.to_string(),
        })
        .collect();
    let linting_errors = crate::linter::lint_commits(&commits, false);

    // When
    let reporting = print_all(&commits, &linting_errors);

    // Then
    insta::assert_snapshot!(snapshot_name, reporting);
}
