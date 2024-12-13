Feature: When the commit title has no space after the colon preceding the Conventional Commits type and scope, it is picked up as a Conventional Commits specification violation.


  Scenario Outline:
    Given the context and environment are reset.
    When linting the "<commit_message>".
    And the argument --output is set as "JSON".
    Then has no space after the colon preceding the type and scope violation is detected.


    Examples:
      | commit_message                                                                                  |
      | "chore:🌐 fix chinese translations"                                                             |
      | "docs:提交文件"                                                                                     |
      | "fix(deps):update dependency gitmojis to v3"                                                    |
      | "fix(deps):update dependency gitmojis to v3\n\n"                                                |
      | "test:making test assertion stricter\n\nCo-authored-by: Renovate Bot <bot@renovateapp.com>\n\n" |
