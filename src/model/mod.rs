#[derive(Debug, Clone, PartialEq)]
#[allow(non_camel_case_types)]
pub enum LintingError {
    NON_ANGULAR_TYPE,
    EMPTY_SCOPE,
    NON_CONVENTIONAL_COMMITS_SPECIFICATION,
    PRECEDING_WHITESPACE,
}

#[derive(Debug)]
pub struct Commit {
    pub oid: git2::Oid,
    pub message: String,
}
