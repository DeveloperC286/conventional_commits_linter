import os


from util import execute_command
from subprocess import Popen, PIPE, STDOUT
from behave import *


@when(
    'conventional_commits_linter is called with the --from-commit-hash argument set as "{from_commit_hash}".')
def execute_conventional_commits_linter(context, from_commit_hash):
    current_directory = os.getcwd()

    conventional_commits_linter_command = current_directory + \
        "/../target/debug/conventional_commits_linter --from-commit-hash " + from_commit_hash

    os.chdir(context.temporary_directory.name)
    (context.exit_code, context.returned_version) = execute_command(
        conventional_commits_linter_command)

    os.chdir(current_directory)


@then('the linting passes.')
def then_linting_passes(context):
    assert int(context.exit_code) == 0


@then('the linting fails.')
def then_linting_fails(context):
    assert int(context.exit_code) != 0