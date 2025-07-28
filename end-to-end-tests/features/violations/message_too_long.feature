Feature: When the commit title exceeds the configured maximum length, it is picked up as a linting violation.


  Scenario: Long message fails with default limit then passes with increased limit
    Given the context and environment are reset.
    When linting the "feat: this is a very long commit message that exceeds fifty characters".
    And the argument --output is set as "JSON".
    Then a message too long violation is detected.
    Given the context and environment are reset.
    When linting the "feat: this is a very long commit message that exceeds fifty characters".
    And the argument --max-commit-title-length is set to "80".
    Then the linting passes.


  Scenario: Short message passes with default limit then fails with decreased limit
    Given the context and environment are reset.
    When linting the "feat: add new feature".
    Then the linting passes.
    Given the context and environment are reset.
    When linting the "feat: add new feature".
    And the argument --max-commit-title-length is set to "15".
    And the argument --output is set as "JSON".
    Then a message too long violation is detected.