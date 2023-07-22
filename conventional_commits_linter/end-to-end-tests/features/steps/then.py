import re
from behave import *

from utilities import is_json, execute_conventional_commits_linter
from assertions import *


@then('the linting passes.')
def then_linting_passes(context):
    # When
    execute_conventional_commits_linter(context)

    # Then
    assert_empty(context.stdout)
    assert_empty(context.stderr)
    assert_successful(context.exit_code)


@then('the linting fails.')
def then_linting_fails(context):
    # When
    execute_conventional_commits_linter(context)

    # Then
    assert_unsuccessful(context.exit_code)


@then('their is a could not find reference "{reference}" error.')
def then_could_not_find_reference_error(context, reference):
    # Given
    could_not_find_reference_error = f" ERROR conventional_commits_linter_lib::commits > Could not find a reference with the name \"{reference}\".\n"

    # When/Then
    then_linting_fails(context)

    # Then
    assert_error(context.stderr, could_not_find_reference_error)


@then('their is a could not find commit hash "{commit_hash}" error.')
def then_could_not_find_commit_hash_error(context, commit_hash):
    # Given
    could_not_find_commit_hash_error = f" ERROR conventional_commits_linter_lib::commits > Can not find a commit with the hash '{commit_hash}'.\n"

    # When/Then
    then_linting_fails(context)

    # Then
    assert_error(context.stderr, could_not_find_commit_hash_error)


@then('their is a missing from argument error.')
def then_missing_from_argument_error(context):
    # Given
    missing_from_argument_error = "error: the following required arguments were not provided:\n  <--from-stdin|--from-reference <FROM_REFERENCE>|--from-commit-hash <FROM_COMMIT_HASH>>\n\nUsage: conventional_commits_linter <--from-stdin|--from-reference <FROM_REFERENCE>|--from-commit-hash <FROM_COMMIT_HASH>>\n\nFor more information, try '--help'.\n"

    # When/Then
    then_linting_fails(context)

    # Then
    assert_error(context.stderr, missing_from_argument_error)


@then('their is a conflicting from arguments error.')
def then_conflicting_from_arguments_error(context):
    # Given
    conflicting_from_commit_hash_and_from_stdin_error = "error: the argument '--from-commit-hash <FROM_COMMIT_HASH>' cannot be used with '--from-stdin'\n\nUsage: conventional_commits_linter <--from-stdin|--from-reference <FROM_REFERENCE>|--from-commit-hash <FROM_COMMIT_HASH>>\n\nFor more information, try '--help'.\n"
    conflicting_from_reference_and_from_commit_hash_error = "error: the argument '--from-reference <FROM_REFERENCE>' cannot be used with '--from-commit-hash <FROM_COMMIT_HASH>'\n\nUsage: conventional_commits_linter <--from-stdin|--from-reference <FROM_REFERENCE>|--from-commit-hash <FROM_COMMIT_HASH>>\n\nFor more information, try '--help'.\n"
    conflicting_from_reference_and_from_stdin_error = "error: the argument '--from-reference <FROM_REFERENCE>' cannot be used with '--from-stdin'\n\nUsage: conventional_commits_linter <--from-stdin|--from-reference <FROM_REFERENCE>|--from-commit-hash <FROM_COMMIT_HASH>>\n\nFor more information, try '--help'.\n"

    # When/Then
    then_linting_fails(context)

    # Then
    assert_in_errors(context.stderr,
                     [conflicting_from_commit_hash_and_from_stdin_error,
                      conflicting_from_reference_and_from_commit_hash_error,
                      conflicting_from_reference_and_from_stdin_error])


@then('standard output is not empty.')
def then_standard_output_not_empty(context):
    assert context.stdout != ""


@then('standard output is empty.')
def then_standard_output_empty(context):
    assert_empty(context.stdout)


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
    could_not_find_shortened_commit_hash_error = f" ERROR conventional_commits_linter_lib::commits > No commit hashes start with the provided short commit hash \"{shortened_commit_hash}\".\n"

    # When/Then
    then_linting_fails(context)

    # Then
    assert_error(context.stderr, could_not_find_shortened_commit_hash_error)


@then(
    'their is a ambiguous shortened commit hash "{shortened_commit_hash}" error.')
def then_ambiguous_shortened_commit_hash_error(context, shortened_commit_hash):
    # Given
    ambiguous_shortened_commit_hash_error = re.compile(
        f"^ ERROR conventional_commits_linter_lib::commits > Ambiguous short commit hash, the commit hashes [[]({shortened_commit_hash}[a-f0-9]*(, )?)*[]] all start with the provided short commit hash \"{shortened_commit_hash}\".\n$")

    # When/Then
    then_linting_fails(context)

    # Then
    assert_regex(context.stderr, ambiguous_shortened_commit_hash_error)


@then('their is a no commits error.')
def then_no_commits_error(context):
    # Given
    no_commits_error = " ERROR conventional_commits_linter_lib::commits > No Git commits within the provided range.\n"

    # When/Then
    then_linting_fails(context)

    # Then
    assert_error(context.stderr, no_commits_error)
