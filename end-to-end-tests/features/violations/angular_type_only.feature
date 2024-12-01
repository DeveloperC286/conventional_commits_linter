Feature: With the allow Angular types only flag, non-Angular types are picked up as violations.


  Scenario Outline:
    Given the context and environment are reset.
    When the flag --from-stdin is set and the standard input is "<standard_input>".
    Then the linting passes.
    When the flag --allow-angular-type-only is set.
    And the argument --output is set as "JSON".
    Then a non-Angular type violation is detected.


    Examples:
      | standard_input                      |
      | "lint: clean up mutex returns\n\n"  |
      | "composer: updated the packages"    |
      | "major: release v3 (merge #51)\n\n" |


  Scenario Outline:
    Given the context and environment are reset.
    When the flag --from-stdin is set and the standard input is "<standard_input>".
    And the flag --allow-angular-type-only is set.
    Then the linting passes.


    Examples:
      | standard_input                      |
      | "chore: update dependencies\n"      |
      | "chore(release): 6.5.0"             |
