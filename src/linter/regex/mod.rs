pub const PRECEDING_WHITESPACE: &str = "^([[:space:]])";
pub const OPTIONAL_EXCLAMATION: &str = "(!)?";
pub const ANGULAR_TYPE: &str = "(revert|build|ci|docs|feat|fix|perf|refactor|style|test)";
pub const EMPTY_SCOPE: &str = r"\(([[:space:]])*\)";
pub const TYPE: &str = r"([[:alpha:]])*";
pub const OPTIONAL_SCOPE: &str = r"(\([[:alpha:]]+\))?";
pub const OPTIONAL_EMPTY_SCOPE_OR_SCOPE: &str = r"(\(.*\))?";

lazy_static! {
    pub static ref OPTIONAL_PRECEDING_WHITESPACE: String = format!("{}*", PRECEDING_WHITESPACE);
}
