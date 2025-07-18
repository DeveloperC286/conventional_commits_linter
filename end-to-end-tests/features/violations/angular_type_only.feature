Feature: With the allow Angular types only flag, non-Angular types are picked up as violations.


  Scenario Outline:
    Given the context and environment are reset.
    When linting the "<commit_message>".
    Then the linting passes.
    When the flag --type is set as "angular".
    And the argument --output is set as "JSON".
    Then a non-Angular type violation is detected.


    Examples:
      | commit_message                      |
      | "lint: clean up mutex returns\n\n"  |
      | "composer: updated the packages"    |
      | "major: release v3 (merge #51)\n\n" |


  Scenario Outline:
    Given the context and environment are reset.
    When linting the "<commit_message>".
    And the flag --type is set as "angular".
    Then the linting passes.


    Examples:
      | commit_message                 |
      | "chore: update dependencies\n" |
      | "chore(release): 6.5.0"        |
