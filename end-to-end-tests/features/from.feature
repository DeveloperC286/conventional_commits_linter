Feature: Ensure input on what to lint or where to lint from is provided correctly.


  Scenario Outline: You must provide either a reference, a commit hash or standard input.
    Given the repository "<repository>" is cloned and checked out at the commit "<checkout_commit>".
    Then their is a missing from argument error.


    Examples:
      | repository                         | checkout_commit                          |
      | https://github.com/yargs/yargs.git | 0f810245494ccf13a35b7786d021b30fc95ecad5 |


  Scenario Outline: You can not provide both a reference and a commit hash.
    Given the repository "<repository>" is cloned and checked out at the commit "<checkout_commit>".
    When the argument --from-reference is provided as "<from_reference>".
    And the argument --from-commit-hash is provided as "<from_commit_hash>".
    Then their is a conflicting from arguments error.


    Examples:
      | repository                         | checkout_commit                          | from_commit_hash                         | from_reference |
      | https://github.com/yargs/yargs.git | 0f810245494ccf13a35b7786d021b30fc95ecad5 | 028b50d5ced3b41a8dccf74107dbfc7065052a5d | v15.4.0        |


  Scenario Outline: You can not provide both a reference and standard input.
    Given the repository "<repository>" is cloned and checked out at the commit "<checkout_commit>".
    When the argument --from-reference is provided as "<from_reference>".
    And the standard input is "<standard_input>".
    Then their is a conflicting from arguments error.


    Examples:
      | repository                         | checkout_commit                          | from_reference | standard_input               |
      | https://github.com/yargs/yargs.git | 0f810245494ccf13a35b7786d021b30fc95ecad5 | v15.4.0        | setup of typescript and jest |


  Scenario Outline: You can not provide both a commit hash and standard input.
    Given the repository "<repository>" is cloned and checked out at the commit "<checkout_commit>".
    When the argument --from-commit-hash is provided as "<from_commit_hash>".
    And the standard input is "<standard_input>".
    Then their is a conflicting from arguments error.


    Examples:
      | repository                         | checkout_commit                          | from_commit_hash                         | standard_input               |
      | https://github.com/yargs/yargs.git | 0f810245494ccf13a35b7786d021b30fc95ecad5 | 028b50d5ced3b41a8dccf74107dbfc7065052a5d | setup of typescript and jest |
