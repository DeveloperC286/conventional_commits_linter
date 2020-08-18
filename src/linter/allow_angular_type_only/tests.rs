use rstest::rstest;

use super::*;

#[rstest(
    commit_message,
    case("bug: ts-node node is not a thing"),
    case("bump: v11.0.0-nightly.20200806"),
    case("feature: add capabilities to app.getLoginItemSettings()"),
    case("cicd: new Jenkinsfile")
)]
fn test_allow_angular_type_only(commit_message: &str) {
    assert_eq!(lint(commit_message), Err(LintingError::NON_ANGULAR_TYPE));
}

#[rstest(
    commit_message,
    case("FIX: v11.0.0-nightly.20200806"),
    case("FeaT: add capabilities to app.getLoginItemSettings()"),
    case("CI: new Jenkinsfile")
)]
fn test_allow_angular_type_only_ignores_casing(commit_message: &str) {
    assert!(lint(commit_message).is_ok());
}
