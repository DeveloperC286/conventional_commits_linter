use rstest::rstest;

use super::*;

#[rstest(
    commit_message,
    case("build: "),
    case("build:"),
    case("chore(ts): "),
    case("chore(deps):"),
    case("feat:"),
    case("feat: \n\n"),
    case("ci:\n\n")
)]
fn test_no_description(commit_message: &str) {
    assert_eq!(lint(commit_message), Err(LintingError::NO_DESCRIPTION));
}

#[rstest(
    commit_message,
    case("gfx::Path -> SkPath\n\nhttps://chromium-review.googlesource.com/c/1392181"),
    case("net::HttpAuthCache::ClearEntriesAddedWithin -> ClearAllEntries\n"),
    case("ui::Display cleanup: some enums --> enum classes\n\nhttps://chromium-review.googlesource.com/915211\n"),
    case("base::LaunchOptions fds_to_remap is no longer a pointer\n\nhttps://codereview.chromium.org/2950153002\n")
)]
fn test_no_description_on_non_conventional_commit(commit_message: &str) {
    assert!(lint(commit_message).is_ok());
}

#[rstest(
    commit_message,
    case("Docs:fix a minor typo"),
    case("ci:Split out appveyor gn builds into separate jobs (#14282)\n\n* Split out appveyor gn into separate jobs"),
    case("ci:Make sure that tests on VSTS get marked as failed if they fail (#14734)\n\n"),
    case("chore:Remove transparency hack (#17128)\n\nThis change undoes the hack that was put in because of a bug in chromium. That has since been fixed in chromium, so this is no longer need, hence removing.\r\nThe \'BrowserWindow\' > \'preserves transparency\' test, validates this working."),
)]
fn test_no_description_on_missing_space(commit_message: &str) {
    assert!(lint(commit_message).is_ok());
}
