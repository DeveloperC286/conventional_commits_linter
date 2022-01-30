Feature: Ensure input on what to lint or where to lint from is provided correctly.


  Scenario Outline: You must provide either a reference, a commit hash or standard input.
    Given the repository "<repository>" is cloned and checked out at the commit "<checkout_commit>".
    Then the linting fails.
    And their is a required arguments missing error.


    Examples:
      | repository                         | checkout_commit                          |
      | https://github.com/yargs/yargs.git | 0f810245494ccf13a35b7786d021b30fc95ecad5 |


  Scenario Outline: You can not provide both a reference and a commit hash.
    Given the repository "<repository>" is cloned and checked out at the commit "<checkout_commit>".
    When the argument --from-reference is provided as "<from_reference>".
    And the argument --from-commit-hash is provided as "<from_commit_hash>".
    Then the linting fails.
    And their is a conflicting reference and commit hash input.


    Examples:
      | repository                         | checkout_commit                          | from_commit_hash                         | from_reference |
      | https://github.com/yargs/yargs.git | 0f810245494ccf13a35b7786d021b30fc95ecad5 | 028b50d5ced3b41a8dccf74107dbfc7065052a5d | v15.4.0        |


  Scenario Outline: You can not provide both a reference and standard input.
    Given the repository "<repository>" is cloned and checked out at the commit "<checkout_commit>".
    When the argument --from-reference is provided as "<from_reference>".
    And the standard input is "<standard_input>".
    Then the linting fails.
    And their is a conflicting reference and standard input input.


    Examples:
      | repository                         | checkout_commit                          | from_reference | standard_input               |
      | https://github.com/yargs/yargs.git | 0f810245494ccf13a35b7786d021b30fc95ecad5 | v15.4.0        | setup of typescript and jest |


  Scenario Outline: You can not provide both a commit hash and standard input.
    Given the repository "<repository>" is cloned and checked out at the commit "<checkout_commit>".
    When the argument --from-commit-hash is provided as "<from_commit_hash>".
    And the standard input is "<standard_input>".
    Then the linting fails.
    And their is a conflicting commit hash and standard input input.


    Examples:
      | repository                         | checkout_commit                          | from_commit_hash                         | standard_input               |
      | https://github.com/yargs/yargs.git | 0f810245494ccf13a35b7786d021b30fc95ecad5 | 028b50d5ced3b41a8dccf74107dbfc7065052a5d | setup of typescript and jest |
