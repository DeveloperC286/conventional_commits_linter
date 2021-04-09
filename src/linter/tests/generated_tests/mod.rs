use super::*;

mod generation;
mod utilities;

#[test]
fn test_generate_commits() {
    let number_of_variants: usize = 4;

    for i in 1..2_usize.pow(number_of_variants as u32) {
        //Given
        let binary_string = format!(
            "{:0desired_length$b}",
            i,
            desired_length = number_of_variants
        );

        let (commits, expected_linting_errors) = generation::generate_commit(
            utilities::is_position_in_binary_string_true(&binary_string, 0),
            utilities::is_position_in_binary_string_true(&binary_string, 1),
            utilities::is_position_in_binary_string_true(&binary_string, 2),
            utilities::is_position_in_binary_string_true(&binary_string, 3),
        );

        //When/Then
        for commit in commits {
            let commit_message = commit.message.clone();
            let commit_oid = commit.oid;
            let linting_errors = lint_commits(&[commit], false);

            assert_eq!(
                expected_linting_errors,
                *linting_errors.get(&commit_oid).unwrap(),
                "Incorrect linting errors for commit message {:?}.",
                commit_message
            );
        }
    }
}
