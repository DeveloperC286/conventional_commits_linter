pub const PRECEDING_WHITESPACE: &str = "^([[:space:]])";
pub const OPTIONAL_EXCLAMATION: &str = "(!)?";
pub const ANGULAR_TYPE: &str = "(revert|build|ci|docs|feat|fix|perf|refactor|style|test)";
pub const EMPTY_SCOPE: &str = r"\(([[:space:]])*\)";
pub const TYPE: &str = r"([[:alpha:]])*";
pub const OPTIONAL_SCOPE: &str = r"(\([[:alpha:]]+\))?";
pub const OPTIONAL_EMPTY_SCOPE_OR_SCOPE: &str = r"(\(.*\))?";

lazy_static! {
    pub static ref OPTIONAL_PRECEDING_WHITESPACE: String = format!("{}*", PRECEDING_WHITESPACE);
    pub static ref IGNORE_TYPE_AND_SCOPE_LINTING_ERRORS: String = format!(
        "{}{}{}{}{}:",
        *crate::linter::regex::OPTIONAL_PRECEDING_WHITESPACE,
        crate::linter::regex::TYPE,
        crate::linter::regex::OPTIONAL_EXCLAMATION,
        crate::linter::regex::OPTIONAL_EMPTY_SCOPE_OR_SCOPE,
        crate::linter::regex::OPTIONAL_EXCLAMATION
    );
}
