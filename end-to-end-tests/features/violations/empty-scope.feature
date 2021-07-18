Feature: Empty scopes are picked up as Conventional Commits specification violation.


  Scenario Outline:
    Given the context and environment are reset.
    When the standard input is "<standard_input>".
    Then the linting fails.
    And a empty scope violation is detected.


    Examples:
      | standard_input                                                                          |
      | "test(): adding stdin scenario variations"                                              |
      | "doc(   ): webpack example (#1436)"                                                     |
      | "chore(): 14.2.0"                                                                       |
      | "chore!(): releasing 14.2.0\n"                                                          |
      | "release(   ): 2.1.0 release\n\nCo-authored-by: Renovate Bot <bot@renovateapp.com>\n\n" |