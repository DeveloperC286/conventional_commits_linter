pub(crate) const PRECEDING_WHITESPACE: &str = "^([[:space:]])";
pub(crate) const OPTIONAL_EXCLAMATION: &str = "(!)?";
pub(crate) const ANGULAR_TYPE: &str = "(revert|build|ci|docs|feat|fix|perf|refactor|style|test)";
pub(crate) const EMPTY_SCOPE: &str = r"\(([[:space:]])*\)";
pub(crate) const TYPE: &str = r"([[:alpha:]])+";
pub(crate) const OPTIONAL_SCOPE: &str = r"(\([[:alpha:]]+\))?";
pub(crate) const OPTIONAL_EMPTY_SCOPE_OR_SCOPE: &str = r"(\(.*\))?";

lazy_static! {
    pub(crate) static ref OPTIONAL_PRECEDING_WHITESPACE: String =
        format!("{PRECEDING_WHITESPACE}*");
    pub(crate) static ref IGNORE_TYPE_AND_SCOPE_LINTING_ERRORS: String = format!(
        "{}{TYPE}{OPTIONAL_EXCLAMATION}{OPTIONAL_EMPTY_SCOPE_OR_SCOPE}{OPTIONAL_EXCLAMATION}:",
        *OPTIONAL_PRECEDING_WHITESPACE,
    );
}
