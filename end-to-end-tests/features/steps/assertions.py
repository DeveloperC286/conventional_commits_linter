import os
from behave import *
from util import execute_command

conflicting_commit_hash_input = "error: The argument '--from-commit-hash <from-commit-hash>' cannot be used with one or more of the other specified arguments\n\nUSAGE:\n    conventional_commits_linter <--from-commit-hash <from-commit-hash>|--from-reference <from-reference>|--from-stdin>\n\nFor more information try --help\n"
conflicting_reference_input = "error: The argument '--from-reference <from-reference>' cannot be used with one or more of the other specified arguments\n\nUSAGE:\n    conventional_commits_linter <--from-commit-hash <from-commit-hash>|--from-reference <from-reference>|--from-stdin>\n\nFor more information try --help\n"
conflicting_standard_input_input = "error: The argument '--from-stdin' cannot be used with one or more of the other specified arguments\n\nUSAGE:\n    conventional_commits_linter <--from-commit-hash <from-commit-hash>|--from-reference <from-reference>|--from-stdin>\n\nFor more information try --help\n"


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


@then('a empty scope violation is detected.')
def then_empty_scope_violation(context):
    empty_scope = "Message - \"" + \
                  context.standard_input.strip('"') + \
                  "\\n\"\n\tX - Commit title does not comply with the Conventional Commits V1.0.0 specification.\n\tX - Commit title has a scope which is empty.\n\n"
    assert context.stdout == empty_scope


@then('a no description after the type and scope violation is detected.')
def then_no_description_violation(context):
    no_description = "Message - \"" + \
                     context.standard_input.strip('"') + \
                     "\\n\"\n\tX - Commit title does not comply with the Conventional Commits V1.0.0 specification.\n\tX - Commit title has no description after the Conventional Commits type and scope.\n\n"
    assert context.stdout == no_description


@then('a no space after the colon preceding the type and scope violation is detected.')
def then_no_space_after_type_violation(context):
    no_space_after_type = "Message - \"" + \
                          context.standard_input.strip('"') + \
                          "\\n\"\n\tX - Commit title does not comply with the Conventional Commits V1.0.0 specification.\n\tX - Commit title has no space after the colon preceding the Conventional Commits type and scope.\n\n"
    assert context.stdout == no_space_after_type


@then('their is a could not find reference "{reference}" error.')
def then_could_not_find_reference(context, reference):
    could_not_find_reference_error = " ERROR conventional_commits_linter::git > Could not find a reference with the name \"" + reference + "\".\n"
    assert context.stdout == could_not_find_reference_error


@then('their is a required arguments missing error.')
def then_required_arguments_missing(context):
    required_arguments_missing = "error: The following required arguments were not provided:\n    <--from-commit-hash <from-commit-hash>|--from-reference <from-reference>|--from-stdin>\n\nUSAGE:\n    conventional_commits_linter [FLAGS] [OPTIONS] <--from-commit-hash <from-commit-hash>|--from-reference <from-reference>|--from-stdin>\n\nFor more information try --help\n"
    assert context.stdout == required_arguments_missing


@then('their is a conflicting reference and commit hash input.')
def then_conflicting_reference_and_commit_hash_input(context):
    assert context.stdout == conflicting_commit_hash_input or context.stdout == conflicting_reference_input


@then('their is a conflicting reference and standard input input.')
def then_conflicting_reference_and_standard_input_input(context):
    assert context.stdout == conflicting_reference_input or context.stdout == conflicting_standard_input_input


@then('their is a conflicting commit hash and standard input input.')
def then_conflicting_reference_and_standard_input_input(context):
    assert context.stdout == conflicting_commit_hash_input or context.stdout == conflicting_standard_input_input
