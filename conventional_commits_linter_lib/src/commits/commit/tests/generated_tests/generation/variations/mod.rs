use super::*;

const PRECEDING_WHITESPACE_VARIATIONS: &[&str] = &["  ", " ", "\t", "\n", "\n\r"];
const NON_PRECEDING_WHITESPACE_VARIATIONS: &[&str] = &[""];

pub(super) fn get_preceding_whitespace_variations(
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

pub(super) const NON_ANGULAR_COMMIT_TYPE_VARIATIONS: &[&str] = &[
    "CICD",
    "cicd",
    "feature",
    "REVERTING",
    "lint",
    "Lint",
    "bug",
    "Bug",
    "BUG",
    "chore",
    "Chore",
];
pub(super) const ANGULAR_COMMIT_TYPE_VARIATIONS: &[&str] = &[
    "REVERT", "revert", "Build", "build", "ci", "CI", "docs", "feat", "FEAT", "fix", "Fix", "perf",
    "refactor", "Refactor", "style", "Style", "test", "TEST",
];

const EMPTY_SCOPE_VARIATIONS: &[&str] = &["()", "(  )"];
const NON_EMPTY_SCOPE_VARIATIONS: &[&str] = &["", "(i18n)", "(parser)", "(strict mode)"];

pub(super) fn get_scope_variations(
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

pub(super) fn get_incorrect_breaking_change_title_variations(
    linting_errors: &mut Vec<LintingError>,
    scope_variations: &'static [&'static str],
    should_generate_incorrect_breaking_change_in_title: bool,
) -> Vec<String> {
    let mut new_scope_variations = vec![];

    match should_generate_incorrect_breaking_change_in_title {
        true => {
            linting_errors.push(LintingError::ExclamationMarkBeforeScope);
            for scope in scope_variations {
                if !scope.is_empty() {
                    new_scope_variations.push(format!("!{scope}"));
                }
            }
        }
        false => {
            for scope in scope_variations {
                new_scope_variations.push(scope.to_string());
                new_scope_variations.push(format!("{scope}!"));
            }
        }
    }

    new_scope_variations
}

pub(super) fn get_after_type_variation(
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

pub(super) fn get_description_variations(
    linting_errors: &mut Vec<LintingError>,
    should_not_generate_description: bool,
) -> &'static [&'static str] {
    match should_not_generate_description {
        true => {
            linting_errors.push(LintingError::NoDescriptionAfterTypeAndScope);
            NON_DESCRIPTION_VARIATIONS
        }
        false => DESCRIPTION_VARIATIONS,
    }
}

const DESCRIPTION_TERMINATION_VARIATIONS: &[&str] = &["\n\n"];
const NON_DESCRIPTION_TERMINATION_VARIATIONS: &[&str] = &[""];

pub(super) fn get_description_termination_variations(
    should_generate_description_termination: bool,
) -> &'static [&'static str] {
    match should_generate_description_termination {
        true => DESCRIPTION_TERMINATION_VARIATIONS,
        false => NON_DESCRIPTION_TERMINATION_VARIATIONS,
    }
}

const BODY_VARIATIONS: &[&str] = &[
    "Helps license scanning tools like https://github.com/licensee/licensee\r\nto successfully detect that this is an MIT licensed project.",
    "* Group all type definitions and helpers in using modules\r\n* Move .d.ts to typings directory\r\n* Get rid of types directory",
    "closes #706\n",
    "Co-authored-by: Renovate Bot <bot@renovateapp.com>",
    "Co-authored-by: github-actions[bot] <41898282+github-actions[bot]@users.noreply.github.com>\r\nCo-authored-by: Benjamin E. Coe <bencoe@google.com>",
];
const NON_BODY_VARIATIONS: &[&str] = &["", "\n", "\n\n"];

pub(super) fn get_body_variations(should_generate_body: bool) -> &'static [&'static str] {
    match should_generate_body {
        true => BODY_VARIATIONS,
        false => NON_BODY_VARIATIONS,
    }
}
