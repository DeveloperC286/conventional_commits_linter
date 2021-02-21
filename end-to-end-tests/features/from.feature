Feature: Ensure input on what to linting is provided correctly.


  Scenario Outline: You must provide either a tag, a commit hash or standard input.
    Given the repository "<repository>" is cloned and checked out at the commit "<checkout_commit>".
    And the directory is changed to the cloned repository.
    Then the linting fails.
    And their is a required arguments missing error.


    Examples:
      | repository                         | checkout_commit                          |
      | https://github.com/yargs/yargs.git | 0f810245494ccf13a35b7786d021b30fc95ecad5 |


  Scenario Outline: You can not provide both a tag and a commit hash.
    Given the repository "<repository>" is cloned and checked out at the commit "<checkout_commit>".
    And the directory is changed to the cloned repository.
    When the argument --from-tag is provided as "<from_tag>".
    And the argument --from-commit-hash is provided as "<from_commit_hash>".
    Then the linting fails.
    And their is a conflicting tag and commit hash input.


    Examples:
      | repository                         | checkout_commit                          | from_commit_hash                         | from_tag |
      | https://github.com/yargs/yargs.git | 0f810245494ccf13a35b7786d021b30fc95ecad5 | 028b50d5ced3b41a8dccf74107dbfc7065052a5d | v15.4.0  |


  Scenario Outline: You can not provide both a tag and standard input.
    Given the repository "<repository>" is cloned and checked out at the commit "<checkout_commit>".
    And the directory is changed to the cloned repository.
    When the argument --from-tag is provided as "<from_tag>".
    And the standard input is "<standard_input>".
    Then the linting fails.
    And their is a conflicting tag and standard input input.


    Examples:
      | repository                         | checkout_commit                          | from_tag | standard_input               |
      | https://github.com/yargs/yargs.git | 0f810245494ccf13a35b7786d021b30fc95ecad5 | v15.4.0  | setup of typescript and jest |

  Scenario Outline: You can not provide both commit hash and standard input.
    Given the repository "<repository>" is cloned and checked out at the commit "<checkout_commit>".
    And the directory is changed to the cloned repository.
    When the argument --from-commit-hash is provided as "<from_commit_hash>".
    And the standard input is "<standard_input>".
    Then the linting fails.
    And their is a conflicting commit hash and standard input input.


    Examples:
      | repository                         | checkout_commit                          | from_commit_hash                         | standard_input               |
      | https://github.com/yargs/yargs.git | 0f810245494ccf13a35b7786d021b30fc95ecad5 | 028b50d5ced3b41a8dccf74107dbfc7065052a5d | setup of typescript and jest |
