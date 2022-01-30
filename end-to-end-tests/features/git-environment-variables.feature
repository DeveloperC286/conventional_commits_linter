Feature: Git environment variables are respected and used instead of using the current working directory.


  Scenario Outline:
    Given the repository "<repository>" is cloned and checked out at the commit "<checkout_commit>".
    When the argument --from-commit-hash is provided as "<from_commit_hash>".
    Then the linting passes.
    Given the GIT_DIR environment variable is set to the cloned repository.
    Then the linting passes.


    Examples:
      | repository                                    | checkout_commit                          | from_commit_hash                         |
      | https://github.com/danielduarte/diffparse.git | df6be23b79af66d3684fb27719020e1ce587f4b8 | 4f6bf53139fe66f61bd05893bcc9de6e96400c5c |


  Scenario Outline:
    Given the repository "<repository>" is cloned and checked out at the commit "<checkout_commit>".
    When the argument --from-commit-hash is provided as "<from_commit_hash>".
    Then the linting fails.
    Given the GIT_DIR environment variable is set to the cloned repository.
    Then the linting fails.


    Examples:
      | repository                                    | checkout_commit                          | from_commit_hash                         |
      | https://gitlab.com/gitlab-org/release-cli.git | 451e0773944e47a4e2678c67691a69cf8934e76e | 6a260e8a74de5c9c85ffd4e2b91632236ea55c3b |


  Scenario Outline:
    Given the repository "<repository>" is cloned and checked out at the commit "<checkout_commit>".
    When the argument --from-reference is provided as "<from_reference>".
    Then the linting passes.
    Given the GIT_DIR environment variable is set to the cloned repository.
    Then the linting passes.


    Examples:
      | repository                         | checkout_commit                          | from_reference |
      | https://github.com/yargs/yargs.git | 0f810245494ccf13a35b7786d021b30fc95ecad5 | v15.4.0        |


  Scenario Outline:
    Given the repository "<repository>" is cloned and checked out at the commit "<checkout_commit>".
    When the argument --from-reference is provided as "<from_reference>".
    Then the linting fails.
    Given the GIT_DIR environment variable is set to the cloned repository.
    Then the linting fails.


    Examples:
      | repository                                     | checkout_commit                          | from_reference        |
      | https://gitlab.com/tortoisegit/tortoisegit.git | 42ffd0e0545202421f3dc658e1e359a01891067a | REL_2.12.0.0_EXTERNAL |
