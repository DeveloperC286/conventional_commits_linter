import re
from behave import *

from utilities import execute_conventional_commits_linter
from assertions import *


@then('the linting passes.')
def assert_linting_passes(context):
    # When
    result = execute_conventional_commits_linter(context)

    # Then
    assert_no_output(result)
    assert_no_errors(result)
    assert_command_successful(result)


@then('the linting fails.')
def assert_linting_fails(context):
    # When
    result = execute_conventional_commits_linter(context)

    # Then
    assert_command_unsuccessful(result)
    return result


@then('their is a could not find reference "{reference}" error.')
def assert_could_not_find_reference_error(context, reference):
    # Given
    could_not_find_reference_error = f" ERROR conventional_commits_linter_lib::commits > Could not find a reference with the name \"{reference}\".\n"

    # When/Then
    result = result = assert_linting_fails(context)

    # Then
    assert_error_equals(result, could_not_find_reference_error)


@then('their is a could not find commit hash "{commit_hash}" error.')
def assert_could_not_find_commit_hash_error(context, commit_hash):
    # Given
    could_not_find_commit_hash_error = f" ERROR conventional_commits_linter_lib::commits > Can not find a commit with the hash '{commit_hash}'.\n"

    # When/Then
    result = assert_linting_fails(context)

    # Then
    assert_error_equals(result, could_not_find_commit_hash_error)


@then('their is a missing from argument error.')
def assert_missing_from_argument_error(context):
    # Given
    missing_from_argument_error = "error: the following required arguments were not provided:\n  <--from-stdin|--from-reference <FROM_REFERENCE>|--from-commit-hash <FROM_COMMIT_HASH>>\n\nUsage: conventional_commits_linter <--from-stdin|--from-reference <FROM_REFERENCE>|--from-commit-hash <FROM_COMMIT_HASH>>\n\nFor more information, try '--help'.\n"

    # When/Then
    result = assert_linting_fails(context)

    # Then
    assert_error_equals(result, missing_from_argument_error)


@then('their is a conflicting from arguments error.')
def assert_conflicting_from_arguments_error(context):
    # Given
    conflicting_from_commit_hash_and_from_stdin_error = "error: the argument '--from-commit-hash <FROM_COMMIT_HASH>' cannot be used with '--from-stdin'\n\nUsage: conventional_commits_linter <--from-stdin|--from-reference <FROM_REFERENCE>|--from-commit-hash <FROM_COMMIT_HASH>>\n\nFor more information, try '--help'.\n"
    conflicting_from_reference_and_from_commit_hash_error = "error: the argument '--from-reference <FROM_REFERENCE>' cannot be used with '--from-commit-hash <FROM_COMMIT_HASH>'\n\nUsage: conventional_commits_linter <--from-stdin|--from-reference <FROM_REFERENCE>|--from-commit-hash <FROM_COMMIT_HASH>>\n\nFor more information, try '--help'.\n"
    conflicting_from_reference_and_from_stdin_error = "error: the argument '--from-reference <FROM_REFERENCE>' cannot be used with '--from-stdin'\n\nUsage: conventional_commits_linter <--from-stdin|--from-reference <FROM_REFERENCE>|--from-commit-hash <FROM_COMMIT_HASH>>\n\nFor more information, try '--help'.\n"

    # When/Then
    result = assert_linting_fails(context)

    # Then
    assert_error_is_one_of(result,
                           [conflicting_from_commit_hash_and_from_stdin_error,
                            conflicting_from_reference_and_from_commit_hash_error,
                            conflicting_from_reference_and_from_stdin_error])


@then('standard output is not empty.')
def assert_standard_output_not_empty(context):
    # When/Then
    result = assert_linting_fails(context)

    # Then
    assert result.stdout != ""


@then('standard output is empty.')
def assert_standard_output_empty(context):
    # When/Then
    result = assert_linting_fails(context)

    # Then
    assert_no_output(result)


@then('standard output is not valid JSON.')
def assert_standard_output_not_valid_json(context):
    # When/Then
    result = assert_linting_fails(context)

    # Then
    assert_invalid_json(result)


@then('standard output is valid JSON.')
def assert_standard_output_valid_json(context):
    # When/Then
    result = assert_linting_fails(context)

    # Then
    assert_valid_json(result)


@then(
    'their is a could not find shortened commit hash "{shortened_commit_hash}" error.')
def assert_could_not_find_shortened_commit_hash_error(
        context, shortened_commit_hash):
    # Given
    could_not_find_shortened_commit_hash_error = f" ERROR conventional_commits_linter_lib::commits > No commit hashes start with the provided short commit hash \"{shortened_commit_hash}\".\n"

    # When/Then
    result = assert_linting_fails(context)

    # Then
    assert_error_equals(result, could_not_find_shortened_commit_hash_error)


@then(
    'their is a ambiguous shortened commit hash "{shortened_commit_hash}" error.')
def assert_ambiguous_shortened_commit_hash_error(
        context, shortened_commit_hash):
    # Given
    ambiguous_shortened_commit_hash_error = re.compile(
        f"^ ERROR conventional_commits_linter_lib::commits > Ambiguous short commit hash, the commit hashes [[]({shortened_commit_hash}[a-f0-9]*(, )?)*[]] all start with the provided short commit hash \"{shortened_commit_hash}\".\n$")

    # When/Then
    result = assert_linting_fails(context)

    # Then
    assert_error_matches_regex(result, ambiguous_shortened_commit_hash_error)


@then('their is a no commits error.')
def assert_no_commits_error(context):
    # Given
    no_commits_error = " ERROR conventional_commits_linter_lib::commits > No Git commits within the provided range.\n"

    # When/Then
    result = assert_linting_fails(context)

    # Then
    assert_error_equals(result, no_commits_error)
