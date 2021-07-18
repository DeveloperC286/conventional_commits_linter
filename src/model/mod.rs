#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum LintingError {
    NonAngularType,
    EmptyScope,
    NonConventionalCommitsSpecification,
    PrecedingWhitespace,
    NoSpaceAfterColonPrecedingTypeAndScope,
    NoDescriptionAfterTypeAndScope,
}

pub struct Commit {
    pub oid: git2::Oid,
    pub message: String,
}
