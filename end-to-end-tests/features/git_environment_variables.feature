Feature: Git environment variables are respected and used instead of using the current working directory.


  Scenario Outline:
    Given the repository "<repository>" is cloned and checked out at the commit "<checkout_commit>".
    When linting from the "<commit_hash>".
    Then the linting passes.
    Given the GIT_DIR environment variable is set to the cloned repository.
    Then the linting passes.


    Examples:
      | repository                                    | checkout_commit                          | commit_hash                              |
      | https://github.com/danielduarte/diffparse.git | df6be23b79af66d3684fb27719020e1ce587f4b8 | 4f6bf53139fe66f61bd05893bcc9de6e96400c5c |


  Scenario Outline: When you provide an invalid commit hash a relevant error message is returned.
    Given the repository "<repository>" is cloned and checked out at the commit "<checkout_commit>".
    When linting from the "<commit_hash>".
    Then their is a could not find commit hash "<commit_hash>" error.
    Given the GIT_DIR environment variable is set to the cloned repository.
    Then their is a could not find commit hash "<commit_hash>" error.


    Examples:
      | repository                                  | checkout_commit                          | commit_hash                              |
      | https://github.com/SergioBenitez/Rocket.git | 549c9241c41320fc5af76b53c2ffc3bd8db88f8c | ecfc2c474575c6cdbc6d273c94c13181bd1dbaa6 |


  Scenario Outline:
    Given the repository "<repository>" is cloned and checked out at the commit "<checkout_commit>".
    When linting from the "<reference>".
    Then the linting passes.
    Given the GIT_DIR environment variable is set to the cloned repository.
    Then the linting passes.


    Examples:
      | repository                         | checkout_commit                          | reference |
      | https://github.com/yargs/yargs.git | 0f810245494ccf13a35b7786d021b30fc95ecad5 | v15.4.0   |


  Scenario Outline: You can also provide the long name and partial names not just the short name.
    Given the repository "<repository>" is cloned and checked out at the commit "<checkout_commit>".
    When linting from the "<reference>".
    Then the linting passes.
    Given the GIT_DIR environment variable is set to the cloned repository.
    Then the linting passes.


    Examples:
      | repository                         | checkout_commit                          | reference         |
      | https://github.com/yargs/yargs.git | 0f810245494ccf13a35b7786d021b30fc95ecad5 | tags/v15.4.0      |
      | https://github.com/yargs/yargs.git | 0f810245494ccf13a35b7786d021b30fc95ecad5 | refs/tags/v15.4.0 |


  Scenario Outline: When you provide an invalid reference a relevant error message is returned.
    Given the repository "<repository>" is cloned and checked out at the commit "<checkout_commit>".
    When linting from the "<reference>".
    Then their is a could not find reference "<reference>" error.
    Given the GIT_DIR environment variable is set to the cloned repository.
    Then their is a could not find reference "<reference>" error.


    Examples:
      | repository                           | checkout_commit                          | reference |
      | https://gitlab.com/Commit451/LabCoat | dcbed0075a41c221b37a3551db51572088fa6e2e | v2.7.4    |
