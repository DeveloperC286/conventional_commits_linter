#[derive(Debug, Clone, PartialEq)]
pub enum LintingError {
    NonAngularType,
    EmptyScope,
    NonConventionalCommitsSpecification,
    PrecedingWhitespace,
    NoSpaceAfterType,
    NoDescription,
}

#[derive(Debug)]
pub struct Commit {
    pub oid: git2::Oid,
    pub message: String,
}
