use super::*;

const PRECEDING_WHITESPACE_VARIATIONS: &[&str] = &["  ", " ", "\t", "\n", "\n\r"];
const NO_PRECEDING_VARIATIONS: &[&str] = &[""];

pub(super) const PRECEDING_VARIATIONS: &[(&[LintingError], &[&str])] = &[
    (
        &[LintingError::PrecedingWhitespace],
        PRECEDING_WHITESPACE_VARIATIONS,
    ),
    (&[], NO_PRECEDING_VARIATIONS),
];

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

const NO_SCOPE_VARIATIONS: &[&str] = &["", "!"];
const EMPTY_SCOPE_VARIATIONS: &[&str] = &["()", "()!", "(  )", "(  )!"];
const EXCLAMATION_MARK_BEFORE_EMPTY_SCOPE_VARIATIONS: &[&str] = &["!()", "!(  )"];
const VALID_SCOPE_VARIATIONS: &[&str] = &["(parser)", "(parser)!"];
const EXCLAMATION_MARK_BEFORE_VALID_SCOPE_VARIATIONS: &[&str] = &["!(parser)"];
const INVALID_SCOPE_VARIATIONS: &[&str] = &["(i18n)", "(i18n)!", "(strict mode)", "(strict mode)!"];
const EXCLAMATION_MARK_BEFORE_INVALID_SCOPE_VARIATIONS: &[&str] = &["!(i18n)", "!(strict mode)"];

pub(super) const SCOPE_VARIATIONS: &[(&[LintingError], &[&str])] = &[
    (&[], NO_SCOPE_VARIATIONS),
    (&[LintingError::EmptyScope], EMPTY_SCOPE_VARIATIONS),
    (
        &[
            LintingError::ExclamationMarkBeforeScope,
            LintingError::EmptyScope,
            LintingError::NonConventionalCommitsSpecification,
        ],
        EXCLAMATION_MARK_BEFORE_EMPTY_SCOPE_VARIATIONS,
    ),
    (&[], VALID_SCOPE_VARIATIONS),
    (
        &[LintingError::ExclamationMarkBeforeScope],
        EXCLAMATION_MARK_BEFORE_VALID_SCOPE_VARIATIONS,
    ),
    // TODO - Specfic non-noun error rather than a generic error.
    (
        &[LintingError::NonConventionalCommitsSpecification],
        INVALID_SCOPE_VARIATIONS,
    ),
    // TODO - Specfic non-noun error rather than a generic error.
    (
        &[
            LintingError::ExclamationMarkBeforeScope,
            LintingError::NonConventionalCommitsSpecification,
        ],
        EXCLAMATION_MARK_BEFORE_INVALID_SCOPE_VARIATIONS,
    ),
];

const NO_SPACE_AFTER_TYPE_VARIATIONS: &[&str] = &[""];
const SPACE_AFTER_TYPE_VARIATIONS: &[&str] = &[" "];

pub(super) const AFTER_TYPE_VARIATIONS: &[(&[LintingError], &[&str])] = &[
    (
        &[LintingError::NoSpaceAfterColonPrecedingTypeAndScope],
        NO_SPACE_AFTER_TYPE_VARIATIONS,
    ),
    (&[], SPACE_AFTER_TYPE_VARIATIONS),
];

const VALID_DESCRIPTION_VARIATIONS: &[&str] = &[
    "expose hideBin helper for CJS \n",
    "release 16.1.0 (#1779)\n",
    "update types for deno ^1.4.0\n",
    "Japanese translation phrasing (#1619)\n",
];
const EMPTY_DESCRIPTION_VARIATIONS: &[&str] = &["", "\n", "\n\n"];

const BODY_VARIATIONS: &[&str] = &[
    "\nHelps license scanning tools like https://github.com/licensee/licensee\r\nto successfully detect that this is an MIT licensed project.",
    "\n* Group all type definitions and helpers in using modules\r\n* Move .d.ts to typings directory\r\n* Get rid of types directory",
    "\ncloses #706\n",
    "\nCo-authored-by: Renovate Bot <bot@renovateapp.com>",
    "\nCo-authored-by: github-actions[bot] <41898282+github-actions[bot]@users.noreply.github.com>\r\nCo-authored-by: Benjamin E. Coe <bencoe@google.com>",
];

pub(super) const DESCRIPTION_AND_BODY_VARIATIONS: &[(&[LintingError], &[&str], &[&str])] = &[
    (&[], VALID_DESCRIPTION_VARIATIONS, &[]),
    (
        &[LintingError::NoDescriptionAfterTypeAndScope],
        EMPTY_DESCRIPTION_VARIATIONS,
        &[],
    ),
    (&[], VALID_DESCRIPTION_VARIATIONS, BODY_VARIATIONS),
];
