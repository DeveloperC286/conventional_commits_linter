Feature: No space after the colon preceding the type and scope is picked up as a Conventional Commits specification violation.


  Scenario Outline:
    Given the context and environment are reset.
    When the standard input is "<standard_input>".
    Then the linting fails.
    And a no space after the colon preceding the type and scope violation is detected.


    Examples:
      | standard_input                                                                                  |
      | "chore:🌐 fix chinese translations"                                                             |
      | "docs:提交文件"                                                                                     |
      | "fix(deps):update dependency gitmojis to v3"                                                    |
      | "fix(deps):update dependency gitmojis to v3\n\n"                                                |
      | "test:making test assertion stricter\n\nCo-authored-by: Renovate Bot <bot@renovateapp.com>\n\n" |
