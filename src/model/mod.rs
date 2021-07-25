use serde::Serialize;

#[derive(Serialize, Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum LintingError {
    NonConventionalCommitsSpecification,
    PrecedingWhitespace,
    NonAngularType,
    EmptyScope,
    NoSpaceAfterColonPrecedingTypeAndScope,
    NoDescriptionAfterTypeAndScope,
}

pub struct Commit {
    pub oid: git2::Oid,
    pub message: String,
}
