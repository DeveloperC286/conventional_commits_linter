use serde::Serialize;

#[derive(Serialize, Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub(crate) enum LintingError {
    NonConventionalCommitsSpecification,
    PrecedingWhitespace,
    NonAngularType,
    EmptyScope,
    NoSpaceAfterColonPrecedingTypeAndScope,
    NoDescriptionAfterTypeAndScope,
}

pub(crate) struct Commit {
    pub(crate) oid: git2::Oid,
    pub(crate) message: String,
}
