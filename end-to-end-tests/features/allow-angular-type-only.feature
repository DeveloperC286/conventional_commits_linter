Feature: conventional_commits_linter with the --allow-angular-type-only flag set fails linting on non Angular types.


  Scenario Outline: When linting a repository using non Angular types linting fails.
    Given the repository "<repository>" is cloned and checked out at the commit "<checkout_commit>".
    And the directory is changed to the cloned repository.
    When the argument --from-commit-hash is provided as "<from_commit_hash>".
    And the flag --allow-angular-type-only is set.
    Then the linting fails.


    Examples:
      | repository                                                  | checkout_commit                          | from_commit_hash                         |
      | https://gitlab.com/dmfay/massive-js.git                     | 673d3b14084cd9bd6d36d1f7e4c1f23e5a59b17c | 4efdac62f53cdce02776f7af4c06ee83d7229ec0 |
      | https://gitlab.com/dmfay/massive-js.git                     | bbe428445dbd914d0e9fbac2ff1cd0079d8d54b7 | 7f44f00e6559110e23d1f45af5f0cc2bafeceefb |
      | https://github.com/hunwalk/yii2-basic-firestarter.git       | 991899445e63f970dc8d933ad8d9f5bbb4a43fbe | b48c4eaa48edc7e6ded79b70141dca8c3c17a1bc |
      | https://github.com/tunnckoCoreLabs/parse-commit-message.git | b45376332f8cecebd538298621eb4595c9c4100d | 081f097081a05142eacd1f640c02767e3bb2f476 |
