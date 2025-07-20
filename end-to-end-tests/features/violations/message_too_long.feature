Feature: When the commit title exceeds the configured maximum length, it is picked up as a linting violation.


  Scenario: Commit message that passes default length limit but fails with custom limit
    Given the context and environment are reset.
    When linting the "feat: add new feature for handling user authentication".
    Then the linting passes.


  Scenario: Commit message that fails with custom length limit
    Given the context and environment are reset.
    When linting the "feat: add new feature for handling user authentication".
    And the argument --max-commit-title-length is set to "30".
    And the argument --output is set as "JSON".
    Then a message too long violation is detected.


  Scenario: Very long commit message that fails default length limit
    Given the context and environment are reset.
    When linting the "feat: this is a very long commit message that definitely exceeds the default fifty character limit".
    And the argument --output is set as "JSON".
    Then a message too long violation is detected.


  Scenario: Length check can be disabled entirely
    Given the context and environment are reset.
    When linting the "feat: this is a very long commit message that definitely exceeds the default fifty character limit".
    And the argument --max-commit-title-length is set to "0".
    Then the linting passes.