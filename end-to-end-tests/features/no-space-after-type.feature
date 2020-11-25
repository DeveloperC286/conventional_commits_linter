Feature: No space after the type/scope are caught as a Conventional Commits specification violation.


  Scenario Outline:
    Given the context and environment are reset.
    When the standard input is "<standard_input>".
    Then an no space after type violation is found.


    Examples:
      | standard_input                      |
      | "chore:🌐 fix chinese translations" |
      | "docs:提交文件头" |
      | "fix(deps):update dependency gitmojis to v3" |