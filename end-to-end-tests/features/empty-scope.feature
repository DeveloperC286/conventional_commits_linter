Feature: Empty scopes are caught as a Conventional Commits specification violation.


  Scenario Outline:
    Given the context and environment are reset.
    When the standard input is "<standard_input>".
    Then an empty scope violation is found.


    Examples:
      | standard_input                             |
      | "test(): adding stdin scenario variations" |
      | "doc(   ): webpack example (#1436)"        |
      | "chore!(): 14.2.0"                         |
