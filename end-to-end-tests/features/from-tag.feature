Feature: A Git tag can be provided as an argument to indicate where to start linting from.


  Scenario Outline:
    Given the repository "<repository>" is cloned and checked out at the commit "<checkout_commit>".
    And the directory is changed to the cloned repository.
    When the argument --from-tag is provided as "<from_tag>".
    Then the linting passes.


    Examples:
      | repository                         | checkout_commit                          | from_tag |
      | https://github.com/yargs/yargs.git | 0f810245494ccf13a35b7786d021b30fc95ecad5 | v15.4.0  |


  Scenario Outline: When you provide an invalid tag a relevant error message is returned.
    Given the repository "<repository>" is cloned and checked out at the commit "<checkout_commit>".
    And the directory is changed to the cloned repository.
    When the argument --from-tag is provided as "<from_tag>".
    Then the linting fails.
    And their is a could not find tag "<from_tag>" error.


    Examples:
      | repository                         | checkout_commit                          | from_tag |
      | https://github.com/yargs/yargs.git | 0f810245494ccf13a35b7786d021b30fc95ecad5 | 12-0-0   |
