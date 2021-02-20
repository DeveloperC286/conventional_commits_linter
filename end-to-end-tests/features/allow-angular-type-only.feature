Feature: With the allow Angular types only flag, valid Conventional Commits not using Angular types fail linting.


  Scenario Outline:
    Given the context and environment are reset.
    When the standard input is "<standard_input>".
    Then the linting passes.
    When the flag --allow-angular-type-only is set.
    Then the linting fails.


    Examples:
      | standard_input                      |
      | "chore: update dependencies\n"      |
      | "chore(release): 6.5.0"             |
      | "lint: clean up mutex returns\n\n"  |
      | "composer: updated the packages"    |
      | "major: release v3 (merge #51)\n\n" |
