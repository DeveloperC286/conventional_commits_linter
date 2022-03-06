use rstest::rstest;

use super::*;

#[rstest(
    commit_message,
    case("feature: adding zsh auto completion (#1292) "),
    case("cicd: releasing 13.1.0\n\n"),
    case("testing: adding os-locale unit tests (#1270)")
)]
fn test_non_angular_type_which_matches_start(commit_message: &str) {
    assert_eq!(lint(commit_message), Err(LintingError::NonAngularType));
}

#[rstest(
    commit_message,
    case("  feature: adding zsh auto completion (#1292) "),
    case("\tcicd: releasing 13.1.0\n\n"),
    case(" testing: adding os-locale unit tests (#1270)")
)]
fn test_non_angular_type_which_matches_start_with_preceding_whitespace(commit_message: &str) {
    assert_eq!(lint(commit_message), Err(LintingError::NonAngularType));
}

#[rstest(
    commit_message,
    case("feature(breaking)!: adding zsh auto completion (#1292) "),
    case("cicd(tag): releasing 13.1.0\n\n"),
    case("multiple(merge): improved completion for choices")
)]
fn test_non_angular_type_with_scope(commit_message: &str) {
    assert_eq!(lint(commit_message), Err(LintingError::NonAngularType));
}
