use super::*;

mod utilities;

#[test]
fn test_generate_commits() {
    let x: u32 = 2;
    let upper_bound = x.pow(3);

    for i in 1..upper_bound {
        //Given
        let binary_string = format!("{:03b}", i);
        let (commits, expected_linting_errors) = utilities::generate_commit(
            utilities::is_position_in_binary_string_true(&binary_string, 0),
            utilities::is_position_in_binary_string_true(&binary_string, 1),
            utilities::is_position_in_binary_string_true(&binary_string, 2),
        );

        //When/Then
        for commit in commits {
            let commit_message = commit.message.clone();
            let commit_oid = commit.oid.clone();
            let linting_errors = lint_commits(&vec![commit], false);
            println!("Oid : {:?}", commit_oid);
            println!("Message : {:?}", commit_message);
            println!("{:?}", linting_errors);

            assert_eq!(
                expected_linting_errors,
                *linting_errors.get(&commit_oid).unwrap(),
                "Incorrect linting errors for commit message {:?}.",
                commit_message
            );
        }
    }
}
