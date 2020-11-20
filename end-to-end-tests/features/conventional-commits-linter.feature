Feature: conventional_commits_linter without any additional flags/config can lint Git commit messages from a repository.


  Scenario Outline: Valid Conventional Commits pass linting.
    Given the repository "<repository>" is cloned and checked out at the commit "<checkout_commit>".
    And the directory is changed to the cloned repository.
    When the argument --from-commit-hash is provided as "<from_commit_hash>".
    Then the linting passes.


    Examples:
      | repository                                    | checkout_commit                          | from_commit_hash                         |
      | https://github.com/yargs/yargs.git            | 027a6365b737e13116811a8ef43670196e1fa00a | 1f26de809432be9cc6f4f185629f6e5d13236598 |
      | https://github.com/yargs/yargs.git            | 18b0b752424bf560271e670ff95a0f90c8386787 | ecfc2c474575c6cdbc6d273c94c13181bd1dbaa6 |
      | https://github.com/electron/electron.git      | 47f88b65b754613bd4370a6f18a06065c40025d0 | 7e84d3a2c1cb4311370c1068fcacb4e5f2d9dd42 |
      | https://github.com/danielduarte/diffparse.git | df6be23b79af66d3684fb27719020e1ce587f4b8 | 4f6bf53139fe66f61bd05893bcc9de6e96400c5c |


  Scenario Outline: Invalid Conventional Commits fail linting.
    Given the repository "<repository>" is cloned and checked out at the commit "<checkout_commit>".
    And the directory is changed to the cloned repository.
    When the argument --from-commit-hash is provided as "<from_commit_hash>".
    Then the linting fails.


    Examples:
      | repository                                    | checkout_commit                          | from_commit_hash                         |
      | https://github.com/SergioBenitez/Rocket.git   | 549c9241c41320fc5af76b53c2ffc3bd8db88f8c | a115eaa633856eb0b09f4019952f866e6b4ef96d |
      | https://github.com/electron/electron.git      | 8798571a77a4d2a7e073b046d2e8b56caa4d1e68 | 77ee3da77ca853485d62aa77698860cae6a9b02b |
      | https://gitlab.com/gitlab-org/release-cli.git | 451e0773944e47a4e2678c67691a69cf8934e76e | 6a260e8a74de5c9c85ffd4e2b91632236ea55c3b |
