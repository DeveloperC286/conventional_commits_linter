Feature: A from argument is required and one must be provided.


  Scenario Outline: You must provide either a reference, a commit hash or standard input.
    Given the repository "<repository>" is cloned and checked out at the commit "<checkout_commit>".
    Then their is a missing from argument error.


    Examples:
      | repository                         | checkout_commit                          |
      | https://github.com/yargs/yargs.git | 0f810245494ccf13a35b7786d021b30fc95ecad5 |
