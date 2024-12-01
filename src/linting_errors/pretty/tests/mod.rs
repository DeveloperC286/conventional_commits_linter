use rstest::rstest;

use crate::commit_type::CommitType;
use crate::commits::commit::Commit;
use crate::commits::Commits;
use crate::source::Source;

const DEFAULT_COMMIT_TYPE: &CommitType = &CommitType::Any;

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
fn test_pretty_print_from_commit_message(commit_message: &str, snapshot_name: &str) {
    // Given
    let commits = crate::commits::Commits::from_commit_message(commit_message);
    let linting_errors = commits.lint(DEFAULT_COMMIT_TYPE).unwrap();

    // When
    let pretty_print = linting_errors.pretty();

    // Then
    insta::assert_snapshot!(snapshot_name, pretty_print);
}

#[rstest(
    commit_messages,
    snapshot_name,
    case(
        &["feat()!: yargs-parser now throws on invalid combinations of config\n\n", "doc(webpack):webpack example (#1436)"],
        "test_pretty_print_from_git_case_1"
    ),
    case(
        &["refactor(ts support)!: ship yargs.d.ts (#1671)", "feat!: drop support for EOL Node 8 (#1686)"],
        "test_pretty_print_from_git_case_2"
    ),
)]
fn test_pretty_print_from_git(commit_messages: &[&str], snapshot_name: &str) {
    // Given
    let commits: Commits = Commits {
        source: Source::Git,
        commits: commit_messages
            .iter()
            .enumerate()
            .map(|(index, commit_message)| Commit {
                hash: Some(
                    git2::Oid::from_str(&format!("{index}"))
                        .unwrap()
                        .to_string(),
                ),
                message: commit_message.to_string(),
            })
            .collect(),
    };

    let linting_errors = commits.lint(DEFAULT_COMMIT_TYPE).unwrap();

    // When
    let pretty_print = linting_errors.pretty();

    //Then
    insta::assert_snapshot!(snapshot_name, pretty_print);
}
