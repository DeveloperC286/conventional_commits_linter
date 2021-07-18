Feature: With the allow Angular types only flag, non-Angular types are picked up as violations.


  Scenario Outline:
    Given the context and environment are reset.
    When the standard input is "<standard_input>".
    Then the linting passes.
    When the flag --allow-angular-type-only is set.
    Then the linting fails.
    And a non-Angular type violation is detected.


    Examples:
      | standard_input                      |
      | "chore: update dependencies\n"      |
      | "chore(release): 6.5.0"             |
      | "lint: clean up mutex returns\n\n"  |
      | "composer: updated the packages"    |
      | "major: release v3 (merge #51)\n\n" |