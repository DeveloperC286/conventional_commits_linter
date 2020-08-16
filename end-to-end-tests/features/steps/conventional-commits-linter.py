import os


from util import execute_command
from subprocess import Popen, PIPE, STDOUT
from behave import *


@when(
    'the argument --from-commit-hash is set as "{from_commit_hash}".')
def set_from_tag(context, from_commit_hash):
    context.set_arguments += " --from-commit-hash " + from_commit_hash


@when(
    'the argument --from-tag is set as "{from_tag}".')
def set_from_tag(context, from_tag):
    context.set_arguments += " --from-tag " + from_tag


@when(
    'conventional_commits_linter is called with the set arguments.')
def execute_conventional_commits_linter(context):
    current_directory = os.getcwd()

    conventional_commits_linter_command = current_directory + \
        "/../target/debug/conventional_commits_linter " + \
        context.set_arguments

    os.chdir(context.temporary_directory.name)
    (context.exit_code, context.stdout) = execute_command(
        conventional_commits_linter_command)

    os.chdir(current_directory)


@then('the linting passes.')
def then_linting_passes(context):
    assert int(context.exit_code) == 0


@then('the linting fails.')
def then_linting_fails(context):
    assert int(context.exit_code) != 0


@then('the error message is "{error_message}".')
def then_the_error_message_is(context, error_message):
    assert context.stdout.strip() == error_message.strip()