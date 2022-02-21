Feature: Only the first parent of merge commit's are parsed for their Git commit messages.


  Scenario Outline:
    Given the repository "<repository>" is cloned and checked out at the commit "<checkout_commit>".
    When the argument --from-commit-hash is provided as "<from_commit_hash>".
    And the argument --output is set as "JSON".
    Then their are "<default_number_of_commits>" commits failing linting.
    When the argument --git-history-mode is provided as "AllParents".
    Then their are "<all_parent_number_of_commits>" commits failing linting.


    Examples:
      | repository                              | checkout_commit                          | from_commit_hash                         | default_number_of_commits | all_parent_number_of_commits |
      | https://github.com/lambci/awslambda.nim | 0fa0f62bec56eb7e09d8fd3e2a4c31798baba89f | 9c9583952ce709fbc6af3b85200c78bf417e5a11 | 5                         | 6                            |
