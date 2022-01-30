Feature: A Git reference can be provided as an argument to indicate where to start taking the range of commits from till HEAD to lint.


  Scenario Outline:
    Given the repository "<repository>" is cloned and checked out at the commit "<checkout_commit>".
    When the argument --from-reference is provided as "<from_reference>".
    Then the linting passes.


    Examples:
      | repository                         | checkout_commit                          | from_reference |
      | https://github.com/yargs/yargs.git | 0f810245494ccf13a35b7786d021b30fc95ecad5 | v15.4.0        |


  Scenario Outline:
    Given the repository "<repository>" is cloned and checked out at the commit "<checkout_commit>".
    When the argument --from-reference is provided as "<from_reference>".
    Then the linting fails.


    Examples:
      | repository                                     | checkout_commit                          | from_reference        |
      | https://gitlab.com/tortoisegit/tortoisegit.git | 42ffd0e0545202421f3dc658e1e359a01891067a | REL_2.12.0.0_EXTERNAL |


  Scenario Outline: When you provide an invalid reference a relevant error message is returned.
    Given the repository "<repository>" is cloned and checked out at the commit "<checkout_commit>".
    When the argument --from-reference is provided as "<from_reference>".
    Then the linting fails.
    And their is a could not find reference "<from_reference>" error.


    Examples:
      | repository                           | checkout_commit                          | from_reference |
      | https://gitlab.com/Commit451/LabCoat | dcbed0075a41c221b37a3551db51572088fa6e2e | v2.7.4         |
