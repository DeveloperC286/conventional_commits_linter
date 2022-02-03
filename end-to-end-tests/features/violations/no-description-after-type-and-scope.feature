Feature: No descriptions after the type and scope is picked up as a Conventional Commits specification violation.


  Scenario Outline:
    Given the context and environment are reset.
    When the standard input is "<standard_input>".
    And the flag --json is set.
    Then a no description after the type and scope violation is detected.


    Examples:
      | standard_input                                                           |
      | "test:       \n\nCo-authored-by: Renovate Bot <bot@renovateapp.com>\n\n" |
      | "doc(README):    \n\n"                                                   |
      | "chore!(release): \n\n"                                                  |
      | "chore!(release): \n"                                                    |
      | "test:       \nCo-authored-by: Renovate Bot <bot@renovateapp.com>\n\n"   |
