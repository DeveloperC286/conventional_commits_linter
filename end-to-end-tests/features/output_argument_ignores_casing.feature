Feature: The output argument can be provided in any casing.


  Scenario Outline:
    Given the context and environment are reset.
    When linting the ""fix(deps updated): update dependency gitmojis to v3"".
    And the argument --output is set as "<output>".
    Then the linting fails.
    And standard output is valid JSON.


    Examples:
      | output |
      | JSON   |
      | json   |
      | Json   |


  Scenario Outline:
    Given the context and environment are reset.
    When linting the ""fix(deps updated): update dependency gitmojis to v3"".
    And the argument --output is set as "<output>".
    Then the linting fails.
    And standard output is empty.


    Examples:
      | output |
      | QUIET  |
      | quiet  |
      | Quiet  |


  Scenario Outline:
    Given the context and environment are reset.
    When linting the ""fix(deps updated): update dependency gitmojis to v3"".
    And the argument --output is set as "<output>".
    Then the linting fails.
    And standard output is not empty.


    Examples:
      | output |
      | PRETTY |
      | pretty |
      | Pretty |
