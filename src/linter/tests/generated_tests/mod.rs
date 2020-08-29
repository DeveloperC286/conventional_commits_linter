use super::*;

mod utilities;

#[test]
fn test_generate_commits() {
    let x: u32 = 2;
    let upper_bound = x.pow(4);

    for i in 1..upper_bound {
        //Given
        let binary_string = format!("{:04b}", i);
        let (commits, expected_linting_errors) = utilities::generate_commit(
            utilities::is_position_in_binary_string_true(&binary_string, 0),
            utilities::is_position_in_binary_string_true(&binary_string, 1),
            utilities::is_position_in_binary_string_true(&binary_string, 2),
            utilities::is_position_in_binary_string_true(&binary_string, 3),
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
