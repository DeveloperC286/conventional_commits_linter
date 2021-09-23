Feature: Git environment variables are respected and used instead of using the current working directory.


  Scenario Outline:
    Given the repository "<repository>" is cloned and checked out at the commit "<checkout_commit>".
    And the directory is changed to the cloned repository.
    When the argument --from-commit-hash is provided as "<from_commit_hash>".
    Then the linting passes.
    Given the directory is changed to the behave directory.
    And the GIT_DIR environment variable is set to the cloned repository.
    Then the linting passes.


    Examples:
      | repository                                    | checkout_commit                          | from_commit_hash                         |
      | https://github.com/danielduarte/diffparse.git | df6be23b79af66d3684fb27719020e1ce587f4b8 | 4f6bf53139fe66f61bd05893bcc9de6e96400c5c |


  Scenario Outline:
    Given the repository "<repository>" is cloned and checked out at the commit "<checkout_commit>".
    And the directory is changed to the cloned repository.
    When the argument --from-commit-hash is provided as "<from_commit_hash>".
    Then the linting fails.
    Given the directory is changed to the behave directory.
    And the GIT_DIR environment variable is set to the cloned repository.
    Then the linting fails.


    Examples:
      | repository                                    | checkout_commit                          | from_commit_hash                         |
      | https://gitlab.com/gitlab-org/release-cli.git | 451e0773944e47a4e2678c67691a69cf8934e76e | 6a260e8a74de5c9c85ffd4e2b91632236ea55c3b |