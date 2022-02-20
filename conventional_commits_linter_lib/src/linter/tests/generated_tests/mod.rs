use super::*;

mod generation;
#[macro_use]
mod macros;
mod utilities;

#[test]
fn test_non_angular_type_commits_with_no_angular_type_only_assertion() {
    let number_of_variants: usize = 4;
    let allow_angular_type_only = false;

    for i in 1..2_usize.pow(number_of_variants as u32) {
        //Given
        let binary_string = format!(
            "{:0desired_length$b}",
            i,
            desired_length = number_of_variants
        );

        let (commit_messages, expected_linting_errors) =
            generation::generate_non_angular_type_commits(
                utilities::is_position_in_binary_string_true(&binary_string, 0),
                utilities::is_position_in_binary_string_true(&binary_string, 1),
                utilities::is_position_in_binary_string_true(&binary_string, 2),
                utilities::is_position_in_binary_string_true(&binary_string, 3),
            );

        //When/Then
        for commit_message in commit_messages {
            assert_linting_errors_eq!(
                expected_linting_errors,
                lint_commit_message(&commit_message, allow_angular_type_only),
                commit_message
            );
        }
    }
}

#[test]
fn test_angular_type_commits_with_no_angular_type_only_assertion() {
    let number_of_variants: usize = 4;
    let allow_angular_type_only = false;

    for i in 1..2_usize.pow(number_of_variants as u32) {
        //Given
        let binary_string = format!(
            "{:0desired_length$b}",
            i,
            desired_length = number_of_variants
        );

        let (commit_messages, expected_linting_errors) = generation::generate_angular_type_commits(
            utilities::is_position_in_binary_string_true(&binary_string, 0),
            utilities::is_position_in_binary_string_true(&binary_string, 1),
            utilities::is_position_in_binary_string_true(&binary_string, 2),
            utilities::is_position_in_binary_string_true(&binary_string, 3),
        );

        //When/Then
        for commit_message in commit_messages {
            assert_linting_errors_eq!(
                expected_linting_errors,
                lint_commit_message(&commit_message, allow_angular_type_only),
                commit_message
            );
        }
    }
}

#[test]
fn test_non_angular_type_commits_with_angular_type_only_assertion() {
    let number_of_variants: usize = 4;
    let allow_angular_type_only = true;

    for i in 1..2_usize.pow(number_of_variants as u32) {
        //Given
        let binary_string = format!(
            "{:0desired_length$b}",
            i,
            desired_length = number_of_variants
        );

        let (commit_messages, mut expected_linting_errors) =
            generation::generate_non_angular_type_commits(
                utilities::is_position_in_binary_string_true(&binary_string, 0),
                utilities::is_position_in_binary_string_true(&binary_string, 1),
                utilities::is_position_in_binary_string_true(&binary_string, 2),
                utilities::is_position_in_binary_string_true(&binary_string, 3),
            );
        expected_linting_errors.push(LintingError::NonAngularType);

        //When/Then
        for commit_message in commit_messages {
            assert_linting_errors_eq!(
                expected_linting_errors,
                lint_commit_message(&commit_message, allow_angular_type_only),
                commit_message
            );
        }
    }
}

#[test]
fn test_angular_type_commits_with_angular_type_only_assertion() {
    let number_of_variants: usize = 4;
    let allow_angular_type_only = true;

    for i in 1..2_usize.pow(number_of_variants as u32) {
        //Given
        let binary_string = format!(
            "{:0desired_length$b}",
            i,
            desired_length = number_of_variants
        );

        let (commit_messages, expected_linting_errors) = generation::generate_angular_type_commits(
            utilities::is_position_in_binary_string_true(&binary_string, 0),
            utilities::is_position_in_binary_string_true(&binary_string, 1),
            utilities::is_position_in_binary_string_true(&binary_string, 2),
            utilities::is_position_in_binary_string_true(&binary_string, 3),
        );

        //When/Then
        for commit_message in commit_messages {
            assert_linting_errors_eq!(
                expected_linting_errors,
                lint_commit_message(&commit_message, allow_angular_type_only),
                commit_message
            );
        }
    }
}