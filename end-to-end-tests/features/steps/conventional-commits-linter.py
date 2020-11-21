import os
from behave import *
from util import execute_command


@when('the argument --from-commit-hash is provided as "{from_commit_hash}".')
def set_from_commit_hash(context, from_commit_hash):
    context.arguments += " --from-commit-hash " + from_commit_hash + " "


@when('the argument --from-tag is provided as "{from_tag}".')
def set_from_tag(context, from_tag):
    context.arguments += " --from-tag " + from_tag + " "


@when('the standard input is "{standard_input}".')
def set_allow_angular_type_only(context, standard_input):
    context.standard_input = "echo \"" + standard_input + "\" | "
    context.arguments += " --from-stdin "


@when('the flag --allow-angular-type-only is set.')
def set_allow_angular_type_only(context):
    context.arguments += " --allow-angular-type-only "


def execute_conventional_commits_linter(context):
    (context.exit_code, context.stdout) = execute_command(
        context.standard_input + context.conventional_commits_linter_path + context.arguments)
    os.chdir(context.behave_directory)


@then('the linting passes.')
def then_linting_passes(context):
    execute_conventional_commits_linter(context)
    assert int(context.exit_code) == 0


@then('the linting fails.')
def then_linting_fails(context):
    execute_conventional_commits_linter(context)
    assert int(context.exit_code) != 0


@then('the error message is "{error_message}".')
def then_the_error_message_is(context, error_message):
    execute_conventional_commits_linter(context)
    assert starts_with(context.stdout, error_message)


@then('the error message is either "{error_message}" or "{error_message2}".')
def then_the_error_message_is_either(context, error_message, error_message2):
    execute_conventional_commits_linter(context)
    assert starts_with(
        context.stdout,
        error_message) or starts_with(
        context.stdout,
        error_message2)


def starts_with(stdout, error_message):
    return stdout.strip().startswith(error_message.strip())
