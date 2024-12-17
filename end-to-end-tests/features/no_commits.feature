Feature: When the range of commits from to lint are empty, then an error is returned.


  Scenario Outline:
    Given the repository "<repository>" is cloned and checked out at the commit "<checkout_commit>".
    When linting from the "<checkout_commit>".
    Then their is a no commits error.


    Examples:
      | repository                                    | checkout_commit                          |
      | https://github.com/danielduarte/diffparse.git | df6be23b79af66d3684fb27719020e1ce587f4b8 |
