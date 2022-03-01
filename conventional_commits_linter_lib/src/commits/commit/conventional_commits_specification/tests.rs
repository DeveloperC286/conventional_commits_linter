use super::*;

#[test]
fn test_type_must_be_provided() {
    // Given
    let commit_message = ": body\n\n";

    // When
    let result = lint(commit_message);

    // Then
    assert_eq!(
        result,
        Err(LintingError::NonConventionalCommitsSpecification)
    );
}
