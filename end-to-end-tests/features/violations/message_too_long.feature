Feature: When the commit title exceeds the configured maximum length, it is picked up as a linting violation.


  Scenario Outline: Long commit title fails with default limit then passes with increased limit
    Given the context and environment are reset.
    When linting the "<commit_message>".
    And the argument --output is set as "JSON".
    Then a commit title too long violation is detected.
    When linting the "<commit_message>".
    And the argument --max-commit-title-length is set to "80".
    Then the linting passes.

    Examples:
      | commit_message                                                    |
      | feat: this is a very long commit message that exceeds fifty characters |


  Scenario Outline: Short commit title passes with default limit then fails with decreased limit
    Given the context and environment are reset.
    When linting the "<commit_message>".
    Then the linting passes.
    When linting the "<commit_message>".
    And the argument --max-commit-title-length is set to "15".
    And the argument --output is set as "JSON".
    Then a commit title too long violation is detected.

    Examples:
      | commit_message      |
      | feat: add new feature |