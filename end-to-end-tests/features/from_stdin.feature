Feature: A commit message can be provided by standard input rather than from a range of commits from Git.


  Scenario Outline:
    Given the context and environment are reset.
    When the flag --from-stdin is set and the standard input is "<standard_input>".
    Then the linting passes.


    Examples:
      | standard_input                           |
      | "test: adding stdin scenario variations" |
      | "docs: correct spelling of CHANGELOG"    |
      | "fix: minor typos in code"               |
