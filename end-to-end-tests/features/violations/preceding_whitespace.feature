Feature: When the commit title has preceding whitespace characters, it is picked up as a Conventional Commits specification violation.


  Scenario Outline:
    Given the context and environment are reset.
    When linting the "<commit_message>".
    And the argument --output is set as "JSON".
    Then has preceding whitespace characters violation is detected.


    Examples:
      | commit_message                                                                       |
      | " docs: 提交文件"                                                                        |
      | "  fix(deps): update dependency gitmojis to v3\n\n"                                  |
      | "\trelease: 2.1.0 release\n\nCo-authored-by: Renovate Bot <bot@renovateapp.com>\n\n" |
