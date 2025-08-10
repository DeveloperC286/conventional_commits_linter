use std::sync::OnceLock;

pub(super) const PRECEDING_WHITESPACE: &str = "^([[:space:]])";
pub(super) const OPTIONAL_PRECEDING_WHITESPACE: &str = "^([[:space:]])*";
pub(super) const OPTIONAL_EXCLAMATION: &str = "(!)?";
pub(super) const ANGULAR_TYPE: &str =
    "(build|chore|ci|docs|feat|fix|perf|refactor|revert|style|test)";
pub(super) const EMPTY_SCOPE: &str = r"\(([[:space:]])*\)";
pub(super) const TYPE: &str = r"([[:alpha:]])+";
pub(super) const OPTIONAL_SCOPE: &str = r"(\([[:alpha:]]+(-[[:alpha:]]+)*\))?";
pub(super) const EMPTY_SCOPE_OR_SCOPE: &str = r"(\(.*\))";
pub(super) const OPTIONAL_EMPTY_SCOPE_OR_SCOPE: &str = r"(\(.*\))?";

static IGNORE_TYPE_AND_SCOPE_LINTING_ERRORS: OnceLock<String> = OnceLock::new();

pub(super) fn ignore_type_and_scope_linting_errors() -> &'static String {
    IGNORE_TYPE_AND_SCOPE_LINTING_ERRORS.get_or_init(|| {
        format!(
            "{OPTIONAL_PRECEDING_WHITESPACE}{TYPE}{OPTIONAL_EXCLAMATION}{OPTIONAL_EMPTY_SCOPE_OR_SCOPE}{OPTIONAL_EXCLAMATION}:",
        )
    })
}
