Feature: A Git commit hash can be provided as an argument to indicate where to start taking the range of commits from till HEAD to lint.


  Scenario Outline:
    Given the repository "<repository>" is cloned and checked out at the commit "<checkout_commit>".
    When linting from the "<commit_hash>".
    Then the linting passes.


    Examples:
      | repository                                    | checkout_commit                          | commit_hash                              |
      | https://github.com/danielduarte/diffparse.git | df6be23b79af66d3684fb27719020e1ce587f4b8 | 4f6bf53139fe66f61bd05893bcc9de6e96400c5c |


  Scenario Outline: When you provide an invalid commit hash a relevant error message is returned.
    Given the repository "<repository>" is cloned and checked out at the commit "<checkout_commit>".
    When linting from the "<commit_hash>".
    Then their is a could not find commit hash "<commit_hash>" error.


    Examples:
      | repository                                  | checkout_commit                          | commit_hash                              |
      | https://github.com/SergioBenitez/Rocket.git | 549c9241c41320fc5af76b53c2ffc3bd8db88f8c | ecfc2c474575c6cdbc6d273c94c13181bd1dbaa6 |
