Feature: When the commit title has a exclamation mark before the scope, it is picked up as a Conventional Commits specification violation.


  Scenario Outline:
    Given the context and environment are reset.
    When the flag --from-stdin is set and the standard input is "<standard_input>".
    And the argument --output is set as "JSON".
    Then has a exclamation mark before the scope violation is detected.


    Examples:
      | standard_input                                                        |
      | "chore!(release): v12.6.0\n\n"                                        |
      | "feat!(api): send an email to the customer when a product is shipped" |
