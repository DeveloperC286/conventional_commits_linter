Feature: Only the first parent of merge commit's are parsed for their Git commit messages.


  Scenario Outline:
    Given the repository "<repository>" is cloned and checked out at the commit "<checkout_commit>".
    When the argument --from-commit-hash is provided as "<from_commit_hash>".
    And the argument --output is set as "JSON".
    Then their are "<number_of_commits>" commits failing linting.


    Examples:
      | repository                      | checkout_commit                          | from_commit_hash                         | number_of_commits |
      | https://github.com/dcyou/resume | bbf42bd631b3ee85de650f46891619833c920bef | 0a74203f382e06b6f39f5143da1eee0be77d3acb | 4                 |
