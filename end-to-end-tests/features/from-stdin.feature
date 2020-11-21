Feature: Standard input can be used to supply a singular commit to lint.


  Scenario Outline:
    Given the context and environment are reset.
    When the standard input is "<standard_input>".
    Then the linting passes.


    Examples:
      | standard_input                         |
      | test: adding stdin scenario variations |


  Scenario Outline:
    Given the context and environment are reset.
    When the standard input is "<standard_input>".
    Then the linting fails.


    Examples:
      | standard_input               |
      | setup of typescript and jest |
