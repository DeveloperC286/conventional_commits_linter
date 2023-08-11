import json


def assert_successful(exit_code):
    assert exit_code == 0, f"Expected a zero exit code to indicate a successful execution.\nExit code = '{exit_code}'.\n"


def assert_unsuccessful(exit_code):
    assert exit_code != 0, f"Expected a non-zero exit code to indicate a unsuccessful execution\nExit code = '{exit_code}'.\n"


def assert_no_output(context):
    assert context.stdout == "", f"Expected standard output to be empty.\nStandard output = {context.stdout.encode()}.\n"


def assert_no_errors(context):
    assert context.stderr == "", f"Expected standard error to be empty.\nStandard error = {context.stderr.encode()}.\n"


def assert_error_equals(context, error):
    assert context.stderr == error, f"Expected standard error to equal the error.\nStandard error = {context.stderr.encode()}.\nError          = {error.encode()}.\n"


def assert_error_matches_regex(context, regex):
    assert regex.match(
        context.stderr) is not None, f"Expected standard errors to match the regex.\nStandard error = {context.stderr.encode()}.\nRegex          = {regex.pattern.encode()}.\n"


def assert_in_errors(output, errors):
    assert output in errors, f"Expected the output to match an error.\nOutput = {output.encode()}.\nErrors  = {errors}.\n"


def assert_invalid_json(context):
    try:
        json.loads(context.stdout)
        assert False, f"Expected standard output to be invalid JSON.\nStandard output = {context.stdout.encode()}.\n"
    except ValueError as _:
        return


def assert_valid_json(context):
    try:
        json.loads(context.stdout)
    except ValueError as _:
        assert False, f"Expected standard output to be valid JSON.\nStandard output = {context.stdout.encode()}.\n"


def assert_commits_linting_errors(context, commits_linting_errors):
    assert_valid_json(context)
    output = json.loads(context.stdout)

    number_of_commits = len(output)
    assert number_of_commits == len(
        commits_linting_errors), f"The number of commits failing linting are not was expected.\nExpected = {len(commits_linting_errors)}\nActual   = {number_of_commits}\n"

    for i in range(number_of_commits):
        linting_errors = output[i]['linting_errors']
        expected_linting_errors = commits_linting_errors[i]
        assert linting_errors == expected_linting_errors, f"The linting errors for the commit are not was expected.\nExpected = {expected_linting_errors}\nActual   = {linting_errors}\n"
