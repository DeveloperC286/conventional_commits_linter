Feature: With JSON enabled when linting errors are encountered they are printed out in a JSON format and not the default pretty more human readable console format.

  Scenario Outline:
    Given the context and environment are reset.
    When linting the "<commit_message>".
    Then the linting fails.
    And standard output is not valid JSON.
    When the argument --output is set as "JSON".
    Then the linting fails.
    And standard output is valid JSON.


    Examples:
      | commit_message                                                                                  |
      | "chore:🌐 fix chinese translations"                                                             |
      | "fix(deps updated): update dependency gitmojis to v3"                                           |
      | "test making test assertion stricter\n\nCo-authored-by: Renovate Bot <bot@renovateapp.com>\n\n" |
