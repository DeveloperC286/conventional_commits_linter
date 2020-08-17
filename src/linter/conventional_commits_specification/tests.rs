use rstest::rstest;

use super::*;

#[rstest(
    commit_message,
    case("feat(deps)!: yargs-parser now throws on invalid combinations of config (\n\n"),
    case("test: add additional test for 1459"),
    case("fix: stop-parse was not being respected by commands (#1459)"),
    case("chore!: drop Node 6 support (#1461)"),
    case("refactor!: remove package.json-based parserConfiguration (#1460)"),
    case("doc(webpack): webpack example (#1436)"),
    case("chore(release): 14.2.0"),
    case("feature: support array of examples (#1682)")
)]
fn test_valid_conventional_commits_specification(commit_message: &str) {
    assert!(lint(commit_message));
}

#[rstest(
    commit_message,
    case(": fix spacing in releae-please.yaml\n\n"),
    case(": tsify lib/command (#1654)"),
    case("Advanced docs typo 'comand' (#1506)"),
    case("Update advance.md: it's -> its (#1499)"),
    case("The first two examples of advanced.md fail silently (#1498) ")
)]
fn test_type_is_required(commit_message: &str) {
    assert_eq!(lint(commit_message), false);
}

#[rstest(
    commit_message,
    case("chore123: describe commandDir() parameters (#1540) (#1613)"),
    case("release 15.4.0: (#1635)"),
    case("fix-deps: Update os-locale to avoid security vulnerability (#1270)")
)]
fn test_type_as_noun_is_required(commit_message: &str) {
    assert_eq!(lint(commit_message), false);
}

#[rstest(
    commit_message,
    case("feat: zsh auto completion (#1292) "),
    case("feat(completion): zsh auto completion (#1292) "),
    case("chore(release): 13.1.0"),
    case("chore: 13.1.0"),
    case("fix: Update os-locale to avoid security vulnerability (#1270)"),
    case("fix(deps): Update os-locale to avoid security vulnerability (#1270)")
)]
fn test_scope_is_allowed_and_optional(commit_message: &str) {
    assert!(lint(commit_message));
}

#[rstest(
    commit_message,
    case("fix(strict mode): report default command unknown arguments (#1626)\n\n"),
    case("feat(yargs-parser): introduce single-digit boolean aliases (#1576)")
)]
fn test_scope_as_noun_is_required(commit_message: &str) {
    assert_eq!(lint(commit_message), false);
}

#[rstest(
    commit_message,
    case("fix!: calling parse multiple times now appropriately maintains state (#\n\n"),
    case("fix: calling parse multiple times now appropriately maintains state (#\n\n"),
    case("refactor(ts)!: ship yargs.d.ts (#1671)"),
    case("refactor(ts): ship yargs.d.ts (#1671)"),
    case("feat!: drop support for EOL Node 8 (#1686)"),
    case("feat: drop support for EOL Node 8 (#1686)")
)]
fn test_scope_and_exclamation_is_optional(commit_message: &str) {
    assert!(lint(commit_message));
}

#[rstest(
    commit_message,
    case("refactor:move to TypeScript release of yargs-parser (#1696)\n\n"),
    case("chore:update dependency @types/mocha to v8 (#1689)"),
    case("chore(deps):update dependency mocha to v8 (#1674)\n\n"),
    case("chore release 15.4.0 (#1635)")
)]
fn test_colon_and_space_is_required(commit_message: &str) {
    assert_eq!(lint(commit_message), false);
}

#[rstest(
    commit_message,
    case("build: "),
    case("build:"),
    case("chore(ts): "),
    case("chore(deps):"),
    case("feat:")
)]
fn test_description_is_required(commit_message: &str) {
    assert_eq!(lint(commit_message), false);
}

#[rstest(
    commit_message,
    case("feat(): zsh auto completion (#1292) "),
    case("chore(): 13.1.0"),
    case("fix(): Update os-locale to avoid security vulnerability (#1270)")
)]
fn test_scope_can_not_be_empty(commit_message: &str) {
    assert_eq!(lint(commit_message), false);
}
