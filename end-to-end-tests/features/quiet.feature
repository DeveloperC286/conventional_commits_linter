Feature: With quiet enabled when linting errors are encountered no output is printed.


  Scenario Outline:
    Given the context and environment are reset.
    When the standard input is "<standard_input>".
    Then the linting fails.
    And standard output is not empty.
    When the flag --quiet is set.
    Then the linting fails.
    And standard output is empty.


    Examples:
      | standard_input                                                                                  |
      | "chore:🌐 fix chinese translations"                                                             |
      | "fix(deps updated): update dependency gitmojis to v3"                                           |
      | "test making test assertion stricter\n\nCo-authored-by: Renovate Bot <bot@renovateapp.com>\n\n" |