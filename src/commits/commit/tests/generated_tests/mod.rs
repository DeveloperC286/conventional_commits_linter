use super::*;
use crate::commit_type::CommitType;

const DEFAULT_COMMIT_TYPE: &CommitType = &CommitType::Any;

mod generation;
#[macro_use]
mod macros;

#[test]
fn test_non_angular_type_commits_with_no_angular_type_only_assertion() {
    for (commit_messages, expected_linting_errors) in
        generation::generate_non_angular_type_commits()
    {
        for commit_message in commit_messages {
            // Given
            let commit = Commit::from_commit_message(&commit_message);

            // When
            let linting_errors = commit.lint(DEFAULT_COMMIT_TYPE);

            // Then
            assert_linting_errors_eq!(expected_linting_errors, linting_errors, commit_message);
        }
    }
}

#[test]
fn test_angular_type_commits_with_no_angular_type_only_assertion() {
    for (commit_messages, expected_linting_errors) in generation::generate_angular_type_commits() {
        for commit_message in commit_messages {
            // Given
            let commit = Commit::from_commit_message(&commit_message);

            // When
            let linting_errors = commit.lint(DEFAULT_COMMIT_TYPE);

            // Then
            assert_linting_errors_eq!(expected_linting_errors, linting_errors, commit_message);
        }
    }
}

#[test]
fn test_non_angular_type_commits_with_angular_type_only_assertion() {
    for (commit_messages, mut expected_linting_errors) in
        generation::generate_non_angular_type_commits()
    {
        expected_linting_errors.push(LintingError::NonAngularType);

        for commit_message in commit_messages {
            // Given
            let commit = Commit::from_commit_message(&commit_message);

            // When
            let linting_errors = commit.lint(&CommitType::Angular);

            // Then
            assert_linting_errors_eq!(expected_linting_errors, linting_errors, commit_message);
        }
    }
}

#[test]
fn test_angular_type_commits_with_angular_type_only_assertion() {
    for (commit_messages, expected_linting_errors) in generation::generate_angular_type_commits() {
        for commit_message in commit_messages {
            // Given
            let commit = Commit::from_commit_message(&commit_message);

            // When
            let linting_errors = commit.lint(&CommitType::Angular);

            // Then
            assert_linting_errors_eq!(expected_linting_errors, linting_errors, commit_message);
        }
    }
}
