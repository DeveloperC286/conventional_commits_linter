Feature: When the commit title has a scope which is not lowercase, it is picked up as a violation when the lowercase-scope flag is enabled.


  Scenario Outline:
    Given the context and environment are reset.
    When linting the "<commit_message>".
    And the argument --lowercase-scope is set.
    And the argument --output is set as "JSON".
    Then has a scope which is not lowercase violation is detected.


    Examples:
      | commit_message                                             |
      | "feat(AUTH): add login functionality"                      |
      | "fix(ApiClient): handle network errors"                    |
      | "docs(README): update installation instructions"           |
      | "test(API-CLIENT): add unit tests"                         |
      | "chore(Test-Utils): refactor helper functions"             |


  Scenario Outline:
    Given the context and environment are reset.
    When linting the "<commit_message>".
    And the argument --lowercase-scope is set.
    And the argument --output is set as "JSON".
    Then the linting passes.


    Examples:
      | commit_message                                             |
      | "feat(auth): add login functionality"                      |
      | "fix(api-client): handle network errors"                   |
      | "docs(readme): update installation instructions"           |
      | "test: add unit tests without scope"                       |
      | "chore(test-utils): refactor helper functions"             |


  Scenario Outline:
    Given the context and environment are reset.
    When linting the "<commit_message>".
    And the argument --output is set as "JSON".
    Then the linting passes.


    Examples:
      | commit_message                                             |
      | "feat(AUTH): add login functionality"                      |
      | "fix(ApiClient): handle network errors"                    |