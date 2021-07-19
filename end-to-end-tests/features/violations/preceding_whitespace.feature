Feature: Preceding whitespace before the type is picked up as a Conventional Commits specification violation.


  Scenario Outline:
    Given the context and environment are reset.
    When the standard input is "<standard_input>".
    Then the linting fails.
    And a preceding whitespace before the type violation is detected.


    Examples:
      | standard_input                                                                       |
      | " docs: 提交文件"                                                                        |
      | "  fix(deps): update dependency gitmojis to v3\n\n"                                  |
      | "\trelease: 2.1.0 release\n\nCo-authored-by: Renovate Bot <bot@renovateapp.com>\n\n" |