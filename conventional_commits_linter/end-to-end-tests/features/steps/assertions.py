import json


def assert_command_successful(result):
    assert result.exit_code == 0, f"Expected a zero exit code to indicate a successful execution.\nExit code = '{result.exit_code}'.\n"


def assert_command_unsuccessful(result):
    assert result.exit_code != 0, f"Expected a non-zero exit code to indicate a unsuccessful execution\nExit code = '{result.exit_code}'.\n"


def assert_output(result):
    assert result.stdout != "", f"Expected standard output to not be empty.\n"


def assert_no_output(result):
    assert result.stdout == "", f"Expected standard output to be empty.\nStandard output = {result.stdout.encode()}.\n"


def assert_no_errors(result):
    assert result.stderr == "", f"Expected standard error to be empty.\nStandard error = {result.stderr.encode()}.\n"


def assert_error_equals(result, error):
    assert result.stderr == error, f"Expected standard error to equal the error.\nStandard error = {result.stderr.encode()}.\nError          = {error.encode()}.\n"


def assert_error_matches_regex(result, regex):
    assert regex.match(
        result.stderr) is not None, f"Expected standard errors to match the regex.\nStandard error = {result.stderr.encode()}.\nRegex          = {regex.pattern.encode()}.\n"


def assert_error_is_one_of(result, errors):
    assert result.stderr in errors, f"Expected standard error to equal one of these errors.\nStandard error = {result.stderr.encode()}.\nErrors         = {errors}.\n"


def assert_invalid_json(result):
    try:
        json.loads(result.stdout)
        assert False, f"Expected standard output to be invalid JSON.\nStandard output = {result.stdout.encode()}.\n"
    except ValueError as _:
        return


def assert_valid_json(result):
    try:
        json.loads(result.stdout)
    except ValueError as _:
        assert False, f"Expected standard output to be valid JSON.\nStandard output = {result.stdout.encode()}.\n"


def assert_number_of_commits(result, expected_number_of_commits):
    assert_valid_json(result)
    output = json.loads(result.stdout)

    number_of_commits = len(output)
    assert number_of_commits == int(
        expected_number_of_commits), f"The number of commits failing linting are not was expected.\nExpected = {expected_number_of_commits}\nActual   = {number_of_commits}\n"


def assert_commits_linting_errors(result, commits_linting_errors):
    assert_valid_json(result)
    output = json.loads(result.stdout)

    assert_number_of_commits(result, len(commits_linting_errors))

    number_of_commits = len(output)
    for i in range(number_of_commits):
        linting_errors = output[i]['linting_errors']
        expected_linting_errors = commits_linting_errors[i]
        assert linting_errors == expected_linting_errors, f"The linting errors for the commit are not was expected.\nExpected = {expected_linting_errors}\nActual   = {linting_errors}\n"
