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
