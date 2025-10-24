Feature: When the commit title has a scope which is not lowercase, it can be detected as a violation with the --lowercase-scope flag.


  Scenario Outline:
    Given the context and environment are reset.
    When linting the "<commit_message>".
    And the argument --output is set as "JSON".
    And the argument --lowercase-scope is set.
    Then has a scope which is not lowercase violation is detected.


    Examples:
      | commit_message                           |
      | "feat(AUTH): add login"                  |
      | "fix(API): handle errors"                |
      | "docs(README): update"                   |
      | "feat(Auth): add login"                  |
      | "fix(ApiClient): handle errors"          |
      | "docs(ReadMe): update"                   |
      | "chore(Deps): update packages"           |
      | "feat(apiClient): add feature"           |
      | "fix(userService): bug fix"              |
      | "test(TestUtils): add test\n\nbody"      |


  Scenario Outline:
    Given the context and environment are reset.
    When linting the "<commit_message>".
    And the argument --output is set as "JSON".
    And the argument --lowercase-scope is set.
    Then linting passes.


    Examples:
      | commit_message                                        |
      | "feat(auth): add login"                               |
      | "fix(api): handle errors"                             |
      | "docs(readme): update"                                |
      | "chore(deps): update packages"                        |
      | "test(user-service): add tests"                       |
      | "feat(api-client): add retry logic"                   |
      | "feat: add feature without scope"                     |
      | "fix: bug fix without scope"                          |
      | "feat(scope-with-hyphens): add feature\n\nwith body"  |


  Scenario Outline:
    Given the context and environment are reset.
    When linting the "<commit_message>".
    And the argument --output is set as "JSON".
    Then linting passes.


    Examples:
      | commit_message          |
      | "feat(AUTH): add login" |
      | "fix(API): handle"      |
      | "docs(README): update"  |
