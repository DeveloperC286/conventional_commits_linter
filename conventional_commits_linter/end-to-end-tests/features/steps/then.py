import re
from behave import *

from utilities import is_json, execute_conventional_commits_linter


@then('the linting passes.')
def then_linting_passes(context):
    # When
    execute_conventional_commits_linter(context)

    # Then
    assert context.stdout == ""
    assert context.stderr == ""
    assert int(context.exit_code) == 0


@then('the linting fails.')
def then_linting_fails(context):
    # When
    execute_conventional_commits_linter(context)

    # Then
    assert int(context.exit_code) != 0


@then('their is a could not find reference "{reference}" error.')
def then_could_not_find_reference_error(context, reference):
    # Given
    could_not_find_reference_error = " ERROR conventional_commits_linter_lib::commits > Could not find a reference with the name \"" + reference + "\".\n"

    # When/Then
    then_linting_fails(context)

    # Then
    assert context.stderr == could_not_find_reference_error


@then('their is a could not find commit hash "{commit_hash}" error.')
def then_could_not_find_commit_hash_error(context, commit_hash):
    # Given
    could_not_find_commit_hash_error = " ERROR conventional_commits_linter_lib::commits > Can not find a commit with the hash '" + commit_hash + "'.\n"

    # When/Then
    then_linting_fails(context)

    # Then
    assert context.stderr == could_not_find_commit_hash_error


@then('their is a missing from argument error.')
def then_missing_from_argument_error(context):
    # Given
    missing_from_argument_error = "error: The following required arguments were not provided:\n" + \
                                  "    <--from-stdin|--from-reference <from-reference>|--from-commit-hash <from-commit-hash>>\n" + \
                                  "\n" + \
                                  "USAGE:\n" + \
                                  "    conventional_commits_linter --output <output> <--from-stdin|--from-reference <from-reference>|--from-commit-hash <from-commit-hash>>\n" + \
                                  "\n" + \
                                  "For more information try --help\n"

    # When/Then
    then_linting_fails(context)

    # Then
    assert context.stderr == missing_from_argument_error


@then('their is a conflicting from arguments error.')
def then_conflicting_from_arguments_error(context):
    # Given
    conflicting_arguments_end = "\n" + \
        "USAGE:\n" + \
        "    conventional_commits_linter --output <output> <--from-stdin|--from-reference <from-reference>|--from-commit-hash <from-commit-hash>>\n" + \
        "\n" + \
        "For more information try --help\n"

    conflicting_from_commit_hash_error = "error: The argument '--from-commit-hash <from-commit-hash>' cannot be used with one or more of the other specified arguments\n" + conflicting_arguments_end
    conflicting_from_reference_error = "error: The argument '--from-reference <from-reference>' cannot be used with one or more of the other specified arguments\n" + conflicting_arguments_end
    conflicting_from_stdin_error = "error: The argument '--from-stdin' cannot be used with one or more of the other specified arguments\n" + conflicting_arguments_end

    # When/Then
    then_linting_fails(context)

    # Then
    assert context.stderr in [
        conflicting_from_commit_hash_error,
        conflicting_from_reference_error,
        conflicting_from_stdin_error]


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


@then(
    'their is a could not find shortened commit hash "{shortened_commit_hash}" error.')
def then_could_not_find_shortened_commit_hash_error(
        context, shortened_commit_hash):
    # Given
    could_not_find_shortened_commit_hash_error = " ERROR conventional_commits_linter_lib::commits > No commit hashes start with the provided short commit hash \"" + \
        shortened_commit_hash + "\".\n"

    # When/Then
    then_linting_fails(context)

    # Then
    assert context.stderr == could_not_find_shortened_commit_hash_error


@then(
    'their is a ambiguous shortened commit hash "{shortened_commit_hash}" error.')
def then_ambiguous_shortened_commit_hash_error(context, shortened_commit_hash):
    # Given
    ambiguous_shortened_commit_hash_error = re.compile(
        '^ ERROR conventional_commits_linter_lib::commits > Ambiguous short commit hash, the commit hashes [[](' +
        shortened_commit_hash +
        '[a-f0-9]*(, )?)*[]] all start with the provided short commit hash "' +
        shortened_commit_hash +
        '".\n$')

    # When/Then
    then_linting_fails(context)

    # Then
    assert ambiguous_shortened_commit_hash_error.match(
        context.stderr) is not None


@then('their is a no commits error.')
def then_no_commits_error(context):
    # Given
    no_commits_error = " ERROR conventional_commits_linter_lib::commits > No Git commits within the provided range.\n"

    # When/Then
    then_linting_fails(context)

    # Then
    assert context.stderr == no_commits_error
