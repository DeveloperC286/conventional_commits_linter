Feature: When the commit title has no description after the Conventional Commits type and scope, it is picked up as a Conventional Commits specification violation.


  Scenario Outline:
    Given the context and environment are reset.
    When the flag --from-stdin is set and the standard input is "<standard_input>".
    And the argument --output is set as "JSON".
    Then has no description after the type and scope violation is detected.


    Examples:
      | standard_input                                                           |
      | "test:       \n\nCo-authored-by: Renovate Bot <bot@renovateapp.com>\n\n" |
      | "doc(README):    \n\n"                                                   |
      | "chore(release)!: \n\n"                                                  |
      | "chore(release)!: \n"                                                    |
      | "test:       \nCo-authored-by: Renovate Bot <bot@renovateapp.com>\n\n"   |
