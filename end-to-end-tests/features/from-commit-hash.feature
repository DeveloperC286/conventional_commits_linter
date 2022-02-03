Feature: A Git commit hash can be provided as an argument to indicate where to start taking the range of commits from till HEAD to lint.


  Scenario Outline:
    Given the repository "<repository>" is cloned and checked out at the commit "<checkout_commit>".
    When the argument --from-commit-hash is provided as "<from_commit_hash>".
    Then the linting passes.


    Examples:
      | repository                                    | checkout_commit                          | from_commit_hash                         |
      | https://github.com/danielduarte/diffparse.git | df6be23b79af66d3684fb27719020e1ce587f4b8 | 4f6bf53139fe66f61bd05893bcc9de6e96400c5c |


  Scenario Outline:
    Given the repository "<repository>" is cloned and checked out at the commit "<checkout_commit>".
    When the argument --from-commit-hash is provided as "<from_commit_hash>".
    Then the linting fails.


    Examples:
      | repository                               | checkout_commit                          | from_commit_hash                         |
      | https://github.com/electron/electron.git | 8798571a77a4d2a7e073b046d2e8b56caa4d1e68 | 77ee3da77ca853485d62aa77698860cae6a9b02b |


  Scenario Outline: When you provide an invalid commit hash a relevant error message is returned.
    Given the repository "<repository>" is cloned and checked out at the commit "<checkout_commit>".
    When the argument --from-commit-hash is provided as "<from_commit_hash>".
    Then their is a could not find commit hash "<from_commit_hash>" error.


    Examples:
      | repository                                  | checkout_commit                          | from_commit_hash                         |
      | https://github.com/SergioBenitez/Rocket.git | 549c9241c41320fc5af76b53c2ffc3bd8db88f8c | ecfc2c474575c6cdbc6d273c94c13181bd1dbaa6 |
