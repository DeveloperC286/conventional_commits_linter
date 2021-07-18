Feature: Standard input can be read and the input is linted as a Git commit message.


  Scenario Outline:
    Given the context and environment are reset.
    When the standard input is "<standard_input>".
    Then the linting passes.


    Examples:
      | standard_input                           |
      | "test: adding stdin scenario variations" |
      | "docs: correct spelling of CHANGELOG"    |
      | "fix: minor typos in code"               |


  Scenario Outline:
    Given the context and environment are reset.
    When the standard input is "<standard_input>".
    Then the linting fails.


    Examples:
      | standard_input                        |
      | "setup of typescript and jest"        |
      | "fix(): setup of typescript and jest" |
      | "feat:initial commit"                 |