use super::*;

mod generation;
mod utilities;

macro_rules! assert_linting_errors_eq {
    ($expected_linting_errors:expr, $actual_linting_errors:expr, $commit_message:expr) => {
        let mut sorted_expected_linting_errors = $expected_linting_errors.clone();
        sorted_expected_linting_errors.sort();
        let mut sorted_actual_linting_errors = $actual_linting_errors.clone();
        sorted_actual_linting_errors.sort();

        assert_eq!(
            sorted_expected_linting_errors, sorted_actual_linting_errors,
            "Incorrect linting errors for commit message {:?}.",
            $commit_message
        )
    };
}

#[test]
fn test_non_angular_type_commits_assertion() {
    let number_of_variants: usize = 4;
    let allow_angular_type_only = false;

    for i in 1..2_usize.pow(number_of_variants as u32) {
        //Given
        let binary_string = format!(
            "{:0desired_length$b}",
            i,
            desired_length = number_of_variants
        );

        let (commits, expected_linting_errors) = generation::generate_various_type_commits(
            utilities::is_position_in_binary_string_true(&binary_string, 0),
            utilities::is_position_in_binary_string_true(&binary_string, 1),
            utilities::is_position_in_binary_string_true(&binary_string, 2),
            utilities::is_position_in_binary_string_true(&binary_string, 3),
        );

        //When/Then
        for commit in commits {
            assert_linting_errors_eq!(
                expected_linting_errors,
                lint_commit(&commit, allow_angular_type_only),
                commit.message
            );
        }
    }
}
