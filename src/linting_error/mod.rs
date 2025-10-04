use serde::Serialize;

/// The representation of a singular linting error a commit message can have.
#[derive(Serialize, Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum LintingError {
    /// Commit title does not comply with the Conventional Commits V1.0.0 specification.
    NonConventionalCommitsSpecification,
    /// Commit title has preceding whitespace characters.
    PrecedingWhitespace,
    /// Commit title does not use an Angular type
    NonAngularType,
    /// Commit title has a exclamation mark before the scope.
    ExclamationMarkBeforeScope,
    /// Commit title has a scope which is empty.
    EmptyScope,
    /// Commit title has no space after the colon preceding the type and scope.
    NoSpaceAfterColonPrecedingTypeAndScope,
    /// Commit title has no description after the type and scope.
    NoDescriptionAfterTypeAndScope,
    /// Commit message subject line exceeds the recommended maximum length.
    CommitTitleTooLong,
    /// Commit title has a scope which is not lowercase.
    NonLowercaseScope,
}
