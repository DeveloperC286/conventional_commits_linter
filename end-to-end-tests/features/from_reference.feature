Feature: A Git reference can be provided as an argument to indicate where to start taking the range of commits from till HEAD to lint.


  Scenario Outline:
    Given the repository "<repository>" is cloned and checked out at the commit "<checkout_commit>".
    When linting from the "<reference>".
    Then the linting passes.


    Examples:
      | repository                         | checkout_commit                          | reference |
      | https://github.com/yargs/yargs.git | 0f810245494ccf13a35b7786d021b30fc95ecad5 | v15.4.0   |


  Scenario Outline: You can also provide the long name and partial names not just the short name.
    Given the repository "<repository>" is cloned and checked out at the commit "<checkout_commit>".
    When linting from the "<full_reference>".
    Then the linting passes.
    Given the arguments are reset.
    When linting from the "<partial_reference>".
    Then the linting passes.
    Given the arguments are reset.
    When linting from the "<short_reference>".
    Then the linting passes.


    Examples:
      | repository                         | checkout_commit                          | full_reference    | partial_reference | short_reference |
      | https://github.com/yargs/yargs.git | 0f810245494ccf13a35b7786d021b30fc95ecad5 | refs/tags/v15.4.0 | tags/v15.4.0      | v15.4.0         |


  Scenario Outline: When you provide an invalid reference a relevant error message is returned.
    Given the repository "<repository>" is cloned and checked out at the commit "<checkout_commit>".
    When linting from the "<reference>".
    Then their is a could not find reference "<reference>" error.


    Examples:
      | repository                           | checkout_commit                          | reference |
      | https://gitlab.com/Commit451/LabCoat | dcbed0075a41c221b37a3551db51572088fa6e2e | v2.7.4    |
