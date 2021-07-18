use super::*;

const PRECEDING_WHITESPACE_VARIATIONS: &[&str] = &["  ", " ", "\t", "\n", "\n\r"];
const NON_PRECEDING_WHITESPACE_VARIATIONS: &[&str] = &[""];

pub fn get_preceding_whitespace_variations(
    linting_errors: &mut Vec<LintingError>,
    should_generate_preceding_whitespace: bool,
) -> &'static [&'static str] {
    match should_generate_preceding_whitespace {
        true => {
            linting_errors.push(LintingError::PrecedingWhitespace);
            PRECEDING_WHITESPACE_VARIATIONS
        }
        false => NON_PRECEDING_WHITESPACE_VARIATIONS,
    }
}

pub const NON_ANGULAR_COMMIT_TYPE_VARIATIONS: &[&str] =
    &["lint", "Lint", "bug", "Bug", "BUG", "chore", "Chore"];
pub const ANGULAR_COMMIT_TYPE_VARIATIONS: &[&str] = &[
    "REVERT", "revert", "Build", "build", "ci", "CI", "docs", "feat", "FEAT", "fix", "Fix", "perf",
    "refactor", "Refactor", "style", "Style", "test", "TEST",
];

const EMPTY_SCOPE_VARIATIONS: &[&str] = &["()", "(  )"];
const NON_EMPTY_SCOPE_VARIATIONS: &[&str] = &["", "(i18n)", "(parser)", "(strict mode)"];

pub fn get_scope_variations(
    linting_errors: &mut Vec<LintingError>,
    should_generate_empty_scope: bool,
) -> &'static [&'static str] {
    match should_generate_empty_scope {
        true => {
            linting_errors.push(LintingError::EmptyScope);
            EMPTY_SCOPE_VARIATIONS
        }
        false => NON_EMPTY_SCOPE_VARIATIONS,
    }
}

pub fn get_after_type_variation(
    linting_errors: &mut Vec<LintingError>,
    should_not_generate_space_after_type: bool,
) -> &'static str {
    match should_not_generate_space_after_type {
        true => {
            linting_errors.push(LintingError::NoSpaceAfterColonPrecedingTypeAndScope);
            ""
        }
        false => " ",
    }
}

const DESCRIPTION_VARIATIONS: &[&str] = &[
    "expose hideBin helper for CJS ",
    "release 16.1.0 (#1779)",
    "update types for deno ^1.4.0",
    "Japanese translation phrasing (#1619)",
];
const NON_DESCRIPTION_VARIATIONS: &[&str] = &["", "\n"];

pub fn get_description_variations(
    linting_errors: &mut Vec<LintingError>,
    should_not_generate_description: bool,
) -> &'static [&'static str] {
    match should_not_generate_description {
        true => {
            linting_errors.push(LintingError::NoDescription);
            NON_DESCRIPTION_VARIATIONS
        }
        false => DESCRIPTION_VARIATIONS,
    }
}
