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
