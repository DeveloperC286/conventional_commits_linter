Feature: No descriptions are caught as a Conventional Commits specification violation.


  Scenario Outline:
    Given the context and environment are reset.
    When the standard input is "<standard_input>".
    Then the linting fails.
    And an no description violation is found.


    Examples:
      | standard_input                                                           |
      | "test:       \n\nCo-authored-by: Renovate Bot <bot@renovateapp.com>\n\n" |
      | "doc(README):    \n\n"                                                   |
      | "chore!(release): \n\n"                                                  |
      | "chore!(release): \n"                                                    |
      | "test:       \nCo-authored-by: Renovate Bot <bot@renovateapp.com>\n\n"   |
