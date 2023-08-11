def assert_successful(exit_code):
    assert exit_code == 0, f"Expected a zero exit code to indicate a successful execution.\nExit code = '{exit_code}'.\n"


def assert_unsuccessful(exit_code):
    assert exit_code != 0, f"Expected a non-zero exit code to indicate a unsuccessful execution\nExit code = '{exit_code}'.\n"


def assert_no_output(context):
    assert context.stdout == "", f"Expected standard output to be empty.\nStandard output = {context.stdout.encode()}.\n"


def assert_no_errors(context):
    assert context.stderr == "", f"Expected standard error to be empty.\nStandard error = {context.stderr.encode()}.\n"


def assert_error(output, error):
    assert output == error, f"Expected the output to equal the error.\nOutput = {output.encode()}.\nError  = {error.encode()}.\n"


def assert_regex(output, regex):
    assert regex.match(
        output) is not None, f"Expected the output to match the regex.\nOutput = {output.encode()}.\nRegex  = {regex.pattern.encode()}.\n"


def assert_in_errors(output, errors):
    assert output in errors, f"Expected the output to match an error.\nOutput = {output.encode()}.\nErrors  = {errors}.\n"
