Feature: A shortened Git commit hash can be provided as an argument to indicate where to start taking the range of commits from till HEAD, instead of a full Git commit hash.


  Scenario Outline: A shortened and full Git commit hash can be used interchangeably.
    Given the repository "<repository>" is cloned and checked out at the commit "<checkout_commit>".
    When the argument --from-commit-hash is provided as "<from_commit_hash>".
	Then the linting passes.
	Given the arguments are reset.
    When the argument --from-commit-hash is provided as "<from_commit_hash>".
	Then the linting passes.


    Examples:
      | repository                                    | checkout_commit                          | from_commit_hash                         | shortened_from_commit_hash |
      | https://github.com/danielduarte/diffparse.git | df6be23b79af66d3684fb27719020e1ce587f4b8 | 4f6bf53139fe66f61bd05893bcc9de6e96400c5c | 4f6bf53 |


  Scenario Outline: The shortened Git commit hash has no matches, so an error is returned.
    Given the repository "<repository>" is cloned and checked out at the commit "<checkout_commit>".
    When the argument --from-commit-hash is provided as "<from_shortened_commit_hash>".
    Then their is a could not find shortened commit hash "<from_shortened_commit_hash>" error.


    Examples:
      | repository                              | checkout_commit                          | from_shortened_commit_hash |
      | https://gitlab.com/dmfay/massive-js.git | 482c364acf5505b81c55245fac0472890d351662 | 3f235ee                    |


  Scenario Outline: The shortened Git commit hash is ambiguous as multiple commit hashes match it, so an error is returned.
    Given the repository "<repository>" is cloned and checked out at the commit "<checkout_commit>".
    When the argument --from-commit-hash is provided as "<from_shortened_commit_hash>".
    Then their is a ambiguous shortened commit hash "<from_shortened_commit_hash>" error.


    Examples:
      | repository                         | checkout_commit                          | from_shortened_commit_hash |
      | https://github.com/yargs/yargs.git | 089417550ef5a5b8ce3578dd2a989191300b64cd | 3f6                        |
