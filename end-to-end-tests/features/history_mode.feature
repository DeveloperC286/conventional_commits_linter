Feature: Specifies how commits are parsed, acceptable values are 'first' to parse only the first parent of merge commits, or 'all' to parse all parents.


  Scenario Outline: Only the first parent of merge commit's are parsed for their Git commit messages.
    Given the repository "<repository>" is cloned and checked out at the commit "<checkout_commit>".
    When linting from the "<commit_hash>".
    And the argument --output is set as "JSON".
    Then their are "<default_number_of_commits>" commits failing linting.
    When the argument --history-mode is provided as "all".
    Then their are "<all_parent_number_of_commits>" commits failing linting.


    Examples:
      | repository                              | checkout_commit                          | commit_hash                              | default_number_of_commits | all_parent_number_of_commits |
      | https://github.com/lambci/awslambda.nim | 0fa0f62bec56eb7e09d8fd3e2a4c31798baba89f | 9c9583952ce709fbc6af3b85200c78bf417e5a11 | 5                         | 6                            |


  Scenario Outline: Only the first parent of merge commit's are parsed for their Git commit messages.
    Given the repository "<repository>" is cloned and checked out at the commit "<checkout_commit>".
    When linting from the "<commit_hash>".
    And the argument --output is set as "JSON".
    And the argument --history-mode is provided as "first".
    Then their are "<number_of_commits>" commits failing linting.


    Examples:
      | repository                      | checkout_commit                          | commit_hash                              | number_of_commits |
      | https://github.com/dcyou/resume | bbf42bd631b3ee85de650f46891619833c920bef | 0a74203f382e06b6f39f5143da1eee0be77d3acb | 4                 |


  Scenario Outline: Only the first parent of merge commit's are parsed for their Git commit messages, by default.
    Given the repository "<repository>" is cloned and checked out at the commit "<checkout_commit>".
    When linting from the "<commit_hash>".
    And the argument --output is set as "JSON".
    Then their are "<number_of_commits>" commits failing linting.


    Examples:
      | repository                      | checkout_commit                          | commit_hash                              | number_of_commits |
      | https://github.com/dcyou/resume | bbf42bd631b3ee85de650f46891619833c920bef | 0a74203f382e06b6f39f5143da1eee0be77d3acb | 4                 |
