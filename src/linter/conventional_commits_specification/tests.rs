use rstest::rstest;

use super::*;

#[rstest(
    commit_message,
    case(": fix spacing in releae-please.yaml\n\n"),
    case(": tsify lib/command (#1654)"),
    case("Advanced docs typo 'comand' (#1506)"),
    case("Update advance.md: it's -> its (#1499)"),
    case("The first two examples of advanced.md fail silently (#1498) ")
)]
fn test_type_is_required(commit_message: &str) {}

#[rstest(
    commit_message,
    case("chore123: describe commandDir() parameters (#1540) (#1613)"),
    case("release 15.4.0: (#1635)"),
    case("fix-deps: Update os-locale to avoid security vulnerability (#1270)")
)]
fn test_type_as_noun_is_required(commit_message: &str) {}

#[rstest(
    commit_message,
    case("fix(strict mode): report default command unknown arguments (#1626)\n\n"),
    case("feat(yargs-parser): introduce single-digit boolean aliases (#1576)")
)]
fn test_scope_as_noun_is_required(commit_message: &str) {}
