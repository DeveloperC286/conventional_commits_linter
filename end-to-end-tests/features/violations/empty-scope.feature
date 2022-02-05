Feature: Empty scopes are picked up as Conventional Commits specification violation.


  Scenario Outline:
    Given the context and environment are reset.
    When the flag --from-stdin is set and the standard input is "<standard_input>".
    And the flag --json is set.
    Then a empty scope violation is detected.


    Examples:
      | standard_input                                                                          |
      | "test(): adding stdin scenario variations"                                              |
      | "doc(   ): webpack example (#1436)"                                                     |
      | "chore(): 14.2.0"                                                                       |
      | "chore!(): releasing 14.2.0\n"                                                          |
      | "release(   ): 2.1.0 release\n\nCo-authored-by: Renovate Bot <bot@renovateapp.com>\n\n" |
