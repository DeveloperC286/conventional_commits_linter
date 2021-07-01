Feature: A Git reference can be provided as an argument to indicate where to start linting from.


  Scenario Outline:
    Given the repository "<repository>" is cloned and checked out at the commit "<checkout_commit>".
    And the directory is changed to the cloned repository.
    When the argument --from-reference is provided as "<from_reference>".
    Then the linting passes.


    Examples:
      | repository                               | checkout_commit                          | from_reference |
      | https://github.com/yargs/yargs.git       | 0f810245494ccf13a35b7786d021b30fc95ecad5 | v15.4.0        |
      | https://github.com/electron/electron.git | 5e033cb7f869f42e93c641fcfddba59fa62742f7 | origin/14-x-y  |
      | https://github.com/electron/electron.git | 42a9d72ce170e5cadfb8bf443886cdbc4bcbcb93 | origin/9-x-y   |

  Scenario Outline: When you provide an invalid reference a relevant error message is returned.
    Given the repository "<repository>" is cloned and checked out at the commit "<checkout_commit>".
    And the directory is changed to the cloned repository.
    When the argument --from-reference is provided as "<from_reference>".
    Then the linting fails.
    And their is a could not find reference "<from_reference>" error.


    Examples:
      | repository                         | checkout_commit                          | from_reference |
      | https://github.com/yargs/yargs.git | 0f810245494ccf13a35b7786d021b30fc95ecad5 | 12-0-0         |
