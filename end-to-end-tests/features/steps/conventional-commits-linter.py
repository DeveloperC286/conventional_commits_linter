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
    context.standard_input = standard_input.strip()
    context.pre_command = "echo " + context.standard_input + " | "
    context.arguments += " --from-stdin "


@when('the flag --allow-angular-type-only is set.')
def set_allow_angular_type_only(context):
    context.arguments += " --allow-angular-type-only "


def execute_conventional_commits_linter(context):
    (context.exit_code, context.stdout) = execute_command(
        context.pre_command + context.conventional_commits_linter_path + context.arguments)
    os.chdir(context.behave_directory)


@then('the linting passes.')
def then_linting_passes(context):
    execute_conventional_commits_linter(context)
    assert int(context.exit_code) == 0


@then('the linting fails.')
def then_linting_fails(context):
    execute_conventional_commits_linter(context)
    assert int(context.exit_code) != 0


@then('an empty scope violation is found.')
def then_empty_scope_violation(context):
    then_linting_fails(context)
    empty_scope = "Message - \"" + \
                  context.standard_input.strip('"') + \
                  "\\n\"\n\tX - Commit title does not comply with the Conventional Commits V1.0.0 specification.\n\tX - Commit title has a scope which is empty.\n\n"
    assert context.stdout == empty_scope


@then('an no description violation is found.')
def then_no_description_violation(context):
    then_linting_fails(context)
    no_description = "Message - \"" + \
                     context.standard_input.strip('"') + \
                     "\\n\"\n\tX - Commit title does not comply with the Conventional Commits V1.0.0 specification.\n\tX - Commit title has no description after the Conventional Commits type and scope.\n\n"
    assert context.stdout == no_description


@then('an no space after type violation is found.')
def then_no_space_after_type_violation(context):
    then_linting_fails(context)
    no_space_after_type = "Message - \"" + \
                          context.standard_input.strip('"') + \
                          "\\n\"\n\tX - Commit title does not comply with the Conventional Commits V1.0.0 specification.\n\tX - Commit title has no space after the colon preceding the Conventional Commits type and scope.\n\n"
    assert context.stdout == no_space_after_type


@then('their is a could not find tag "{tag}" error.')
def then_could_not_find_tag(context, tag):
    execute_conventional_commits_linter(context)
    could_not_find_tag = " ERROR conventional_commits_linter::git > Could not find a tag with the name '" + tag + "'.\n"
    assert context.stdout == could_not_find_tag


@then('their is a required arguments missing error.')
def then_required_arguments_missing(context):
    execute_conventional_commits_linter(context)
    required_arguments_missing = "error: The following required arguments were not provided:\n    <--from-commit-hash <from-commit-hash>|--from-tag <from-tag>|--from-stdin>\n\nUSAGE:\n    conventional_commits_linter [FLAGS] [OPTIONS] <--from-commit-hash <from-commit-hash>|--from-tag <from-tag>|--from-stdin>\n\nFor more information try --help\n"
    assert context.stdout == required_arguments_missing


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
