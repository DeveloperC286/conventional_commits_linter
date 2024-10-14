import json


def assert_command_successful(result):
    assert result.exit_code == 0, "Expected a zero exit code to indicate a successful execution.\n" + \
        f"Exit code = '{result.exit_code}'.\n"


def assert_command_unsuccessful(result):
    assert result.exit_code != 0, "Expected a non-zero exit code to indicate a unsuccessful execution\n" + \
        f"Exit code = '{result.exit_code}'.\n"


def assert_output(result):
    assert result.stdout != "", f"Expected standard output to not be empty.\n"


def assert_no_output(result):
    assert result.stdout == "", "Expected standard output to be empty.\n" + \
        f"Standard output = {result.stdout.encode()}.\n"


def assert_no_errors(result):
    assert result.stderr == "", "Expected standard error to be empty.\n" + \
        f"Standard error = {result.stderr.encode()}.\n"


def assert_error_equals(result, error):
    assert result.stderr == error, "Expected standard error to equal the error.\n" + \
        f"Standard error = {result.stderr.encode()}.\n" + \
        f"Error          = {error.encode()}.\n"


def assert_error_matches_regex(result, regex):
    assert regex.match(result.stderr) is not None, f"Expected standard errors to match the regex.\n" + \
        f"Standard error = {result.stderr.encode()}.\n" + \
        f"Regex          = {regex.pattern.encode()}.\n"


def assert_error_is_one_of(result, errors):
    assert result.stderr in errors, "Expected standard error to equal one of these errors.\n" + \
        f"Standard error = {result.stderr.encode()}.\n" + \
        f"Errors         = {errors}.\n"


def assert_invalid_json(result):
    try:
        json.loads(result.stdout)
        assert False, f"Expected standard output to be invalid JSON.\n" + \
            f"Standard output = {result.stdout.encode()}.\n"
    except ValueError as _:
        return


def assert_valid_json(result):
    try:
        json.loads(result.stdout)
    except ValueError as _:
        assert False, f"Expected standard output to be valid JSON.\n" + \
            f"Standard output = {result.stdout.encode()}.\n"


def assert_number_of_commits(result, expected_number_of_commits):
    assert_valid_json(result)
    output = json.loads(result.stdout)

    number_of_commits = len(output)
    assert number_of_commits == int(
        expected_number_of_commits), f"The number of commits failing linting are not was expected.\n" + \
        f"Expected = {expected_number_of_commits}\n" + \
        f"Actual   = {number_of_commits}\n"


def assert_commits_linting_errors(result, commits_linting_errors):
    assert_valid_json(result)
    output = json.loads(result.stdout)

    assert_number_of_commits(result, len(commits_linting_errors))

    number_of_commits = len(output)
    for i in range(number_of_commits):
        linting_errors = output[i]['linting_errors']
        expected_linting_errors = commits_linting_errors[i]
# fmt: off
        assert linting_errors == expected_linting_errors, f"The linting errors for the commit are not was expected.\n" + \
            f"Expected = {expected_linting_errors}\n" + \
            f"Actual   = {linting_errors}\n"
