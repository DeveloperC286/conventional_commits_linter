#[derive(Debug, Clone, PartialEq)]
#[allow(non_camel_case_types)]
pub enum LintingError {
    NON_ANGULAR_TYPE,
    NON_CONVENTIONAL_COMMITS_SPECIFICATION,
}

#[derive(Debug)]
pub struct Commit {
    pub oid: git2::Oid,
    pub message: String,
}
