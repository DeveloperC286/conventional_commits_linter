use rstest::rstest;

use super::*;

mod generating_utilities;

#[rstest(
    commit_message,
    case("feat(deps)!: yargs-parser now throws on invalid combinations of config (\n\n"),
    case("test: add additional test for 1459"),
    case("fix: stop-parse was not being respected by commands (#1459)"),
    case("chore!: drop Node 6 support (#1461)"),
    case("refactor!: remove package.json-based parserConfiguration (#1460)"),
    case("doc(webpack): webpack example (#1436)"),
    case("chore(release): 14.2.0"),
    case("feature: support array of examples (#1682)"),
    case("feat: zsh auto completion (#1292) "),
    case("feat(completion): zsh auto completion (#1292) "),
    case("chore(release): 13.1.0"),
    case("chore: 13.1.0"),
    case("fix: Update os-locale to avoid security vulnerability (#1270)"),
    case("fix(deps): Update os-locale to avoid security vulnerability (#1270)"),
    case("fix!: calling parse multiple times now appropriately maintains state (#\n\n"),
    case("fix: calling parse multiple times now appropriately maintains state (#\n\n"),
    case("refactor(ts)!: ship yargs.d.ts (#1671)"),
    case("refactor(ts): ship yargs.d.ts (#1671)"),
    case("feat!: drop support for EOL Node 8 (#1686)"),
    case("feat: drop support for EOL Node 8 (#1686)")
)]
fn test_lint_commits_on_valid(commit_message: &str) {
    assert!(lint_commits(
        &vec![Commit {
            oid: git2::Oid::zero(),
            message: commit_message.to_string(),
        }],
        false
    )
    .is_empty());
}

#[test]
fn test_generate_commits() {
    let x: u32 = 2;
    let upper_bound = x.pow(2);

    for i in 1..upper_bound {
        //Given
        let binary_string = format!("{:02b}", i);
        let (commits, expected_linting_errors) = generating_utilities::generate_commit(
            generating_utilities::is_position_in_binary_string_true(&binary_string, 0),
            generating_utilities::is_position_in_binary_string_true(&binary_string, 1),
        );

        //When/Then
        for commit in commits {
            let commit_message = commit.message.clone();
            let commit_oid = commit.oid.clone();
            let linting_errors = lint_commits(&vec![commit], false);

            assert_eq!(
                expected_linting_errors,
                *linting_errors.get(&commit_oid).unwrap(),
                "Incorrect linting errors for commit message {:?}.",
                commit_message
            );
        }
    }
}
