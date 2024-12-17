Feature: A commit message can be provided by standard input rather than from a range of commits from Git.


  Scenario Outline:
    Given the context and environment are reset.
    When linting the "<commit_message>".
    Then the linting passes.


    Examples:
      | commit_message                           |
      | "test: adding stdin scenario variations" |
      | "docs: correct spelling of CHANGELOG"    |
      | "fix: minor typos in code"               |
