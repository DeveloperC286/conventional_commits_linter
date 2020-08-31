Feature: conventional_commits_linter can use the argument --from-tag in the place of the --from-commit-hash argument.


Scenario Outline: When you provide both --from-tag and --from-commit-hash arguments conventional_commits_linter fails.
    Given the repository "<repository>" is cloned and checked out at the commit "<checkout_commit>".
    When the argument --from-commit-hash is set as "<from_commit_hash>".
    When the argument --from-tag is set as "<from_tag>".
    When conventional_commits_linter is called with the set arguments.
    Then the linting fails.
    Then the error message is either "error: The argument '--from-tag <from-tag>' cannot be used with one or more of the other specified arguments" or "error: The argument '--from-commit-hash <from-commit-hash>' cannot be used with one or more of the other specified arguments".


Examples:
    | repository                                    | checkout_commit                          | from_commit_hash                         | from_tag |
    | https://github.com/yargs/yargs.git            | 0f810245494ccf13a35b7786d021b30fc95ecad5 | 028b50d5ced3b41a8dccf74107dbfc7065052a5d | v15.4.0  |


Scenario Outline: When you provide neither --from-tag and --from-commit-hash arguments conventional_commits_linter fails.
    Given the repository "<repository>" is cloned and checked out at the commit "<checkout_commit>".
    When conventional_commits_linter is called with the set arguments.
    Then the linting fails.
    Then the error message is "error: The following required arguments were not provided:".


Examples:
    | repository                                    | checkout_commit                          |
    | https://github.com/yargs/yargs.git            | 0f810245494ccf13a35b7786d021b30fc95ecad5 |


Scenario Outline: You can use the --from-tag argument instead of the --from-commit-hash argument.
    Given the repository "<repository>" is cloned and checked out at the commit "<checkout_commit>".
    When the argument --from-tag is set as "<from_tag>".
    When conventional_commits_linter is called with the set arguments.
    Then the linting passes.


Examples:
    | repository                                    | checkout_commit                          | from_tag |
    | https://github.com/yargs/yargs.git            | 0f810245494ccf13a35b7786d021b30fc95ecad5 | v15.4.0  |


Scenario Outline: When you use the --from-tag argument with an invalid tag name a relevant error message is returned.
    Given the repository "<repository>" is cloned and checked out at the commit "<checkout_commit>".
    When the argument --from-tag is set as "<from_tag>".
    When conventional_commits_linter is called with the set arguments.
    Then the linting fails.
    Then the error message is "ERROR conventional_commits_linter::git > Could not find tag with the name '12-0-0'.".


Examples:
    | repository                                    | checkout_commit                          | from_tag |
    | https://github.com/yargs/yargs.git            | 0f810245494ccf13a35b7786d021b30fc95ecad5 | 12-0-0   |
