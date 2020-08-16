Feature: conventional_commits_linter can use the argument --from-tag in the place of the --from-commit-hash argument.


Scenario Outline: When you provide both --from-tag and --from-commit-hash arguments conventional_commits_linter fails.
    Given the repository "<repository>" is cloned and checked out at the commit "<checkout_commit>".
    When the argument --from-commit-hash is set as "<from_commit_hash>".
    When the argument --from-tag is set as "<from_tag>".
    When conventional_commits_linter is called with the set arguments.
    Then the linting fails.
    Then the error message is "ERROR conventional_commits_linter > Provide either the --from-tag or --from-commit-hash arguement not both.".


Examples:
    | repository                                    | checkout_commit                          | from_commit_hash                         | from_tag |
    | https://github.com/yargs/yargs.git            | 0f810245494ccf13a35b7786d021b30fc95ecad5 | 028b50d5ced3b41a8dccf74107dbfc7065052a5d | v15.4.0  |


Scenario Outline: You can use the --from-tag argument instead of the --from-commit-hash argument.
    Given the repository "<repository>" is cloned and checked out at the commit "<checkout_commit>".
    When the argument --from-tag is set as "<from_tag>".
    When conventional_commits_linter is called with the set arguments.
    Then the linting passes.


Examples:
    | repository                                    | checkout_commit                          | from_tag |
    | https://github.com/yargs/yargs.git            | 0f810245494ccf13a35b7786d021b30fc95ecad5 | v15.4.0  |


Scenario Outline: When you use the --from-tag argument with an invlaid tag name a relevent error message is returned.
    Given the repository "<repository>" is cloned and checked out at the commit "<checkout_commit>".
    When the argument --from-tag is set as "<from_tag>".
    When conventional_commits_linter is called with the set arguments.
    Then the linting fails.
    Then the error message is "ERROR conventional_commits_linter > Could not find tag with the name '12-0-0'.".


Examples:
    | repository                                    | checkout_commit                          | from_tag |
    | https://github.com/yargs/yargs.git            | 0f810245494ccf13a35b7786d021b30fc95ecad5 | 12-0-0   |
