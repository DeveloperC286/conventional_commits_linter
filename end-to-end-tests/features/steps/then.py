from behave import *

from utilities import is_json, execute_conventional_commits_linter

conflicting_commit_hash_input = "error: The argument '--from-commit-hash <from-commit-hash>' cannot be used with one or more of the other specified arguments\n\nUSAGE:\n    conventional_commits_linter <--from-commit-hash <from-commit-hash>|--from-reference <from-reference>|--from-stdin>\n\nFor more information try --help\n"
conflicting_reference_input = "error: The argument '--from-reference <from-reference>' cannot be used with one or more of the other specified arguments\n\nUSAGE:\n    conventional_commits_linter <--from-commit-hash <from-commit-hash>|--from-reference <from-reference>|--from-stdin>\n\nFor more information try --help\n"
conflicting_standard_input_input = "error: The argument '--from-stdin' cannot be used with one or more of the other specified arguments\n\nUSAGE:\n    conventional_commits_linter <--from-commit-hash <from-commit-hash>|--from-reference <from-reference>|--from-stdin>\n\nFor more information try --help\n"


@then('the linting passes.')
def then_linting_passes(context):
    execute_conventional_commits_linter(context)
    assert context.stdout == ""
    assert context.stderr == ""
    assert int(context.exit_code) == 0


@then('the linting fails.')
def then_linting_fails(context):
    execute_conventional_commits_linter(context)
    assert int(context.exit_code) != 0


@then('their is a could not find reference "{reference}" error.')
def then_could_not_find_reference(context, reference):
    could_not_find_reference_error = " ERROR conventional_commits_linter::git > Could not find a reference with the name \"" + reference + "\".\n"
    assert context.stderr == could_not_find_reference_error


@then('their is a could not find commit hash "{commit_hash}" error.')
def then_could_not_find_commit_hash(context, commit_hash):
    could_not_find_commit_hash_error = " ERROR conventional_commits_linter::git > Could not find a commit with the commit hash '" + commit_hash + "'.\n"
    assert context.stderr == could_not_find_commit_hash_error


@then('their is a required arguments missing error.')
def then_required_arguments_missing(context):
    required_arguments_missing = "error: The following required arguments were not provided:\n    <--from-commit-hash <from-commit-hash>|--from-reference <from-reference>|--from-stdin>\n\nUSAGE:\n    conventional_commits_linter [FLAGS] [OPTIONS] <--from-commit-hash <from-commit-hash>|--from-reference <from-reference>|--from-stdin>\n\nFor more information try --help\n"
    assert context.stderr == required_arguments_missing


@then('their is a conflicting reference and commit hash input.')
def then_conflicting_reference_and_commit_hash_input(context):
    assert context.stderr == conflicting_commit_hash_input or context.stderr == conflicting_reference_input


@then('their is a conflicting reference and standard input input.')
def then_conflicting_reference_and_standard_input_input(context):
    assert context.stderr == conflicting_reference_input or context.stderr == conflicting_standard_input_input


@then('their is a conflicting commit hash and standard input input.')
def then_conflicting_reference_and_standard_input_input(context):
    assert context.stderr == conflicting_commit_hash_input or context.stderr == conflicting_standard_input_input


@then('standard output is not empty.')
def then_standard_output_not_empty(context):
    assert context.stdout != ""


@then('standard output is empty.')
def then_standard_output_empty(context):
    assert context.stdout == ""


@then('standard output is not valid JSON.')
def then_standard_output_not_valid_json(context):
    assert not is_json(context.stdout)


@then('standard output is valid JSON.')
def then_standard_output_valid_json(context):
    assert is_json(context.stdout)
