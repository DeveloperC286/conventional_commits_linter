import json
from behave import *

from then import assert_linting_fails
from assertions import *


@then('their are "{expected_number_of_commits}" commits failing linting.')
def assert_number_of_commits_failing_linting(
        context, expected_number_of_commits):
    # When/Then
    result = assert_linting_fails(context)

    # Then
    assert_number_of_commits(result, expected_number_of_commits)


@then('has preceding whitespace characters violation is detected.')
def assert_preceding_whitespace_violation(context):
    # When/Then
    result = assert_linting_fails(context)

    # Then
    assert_commits_linting_errors(
        result, [['NonConventionalCommitsSpecification', 'PrecedingWhitespace']])


@then('a non-Angular type violation is detected.')
def assert_non_angular_type_violation(context):
    # When/Then
    result = assert_linting_fails(context)

    # Then
    assert_commits_linting_errors(
        result, [['NonAngularType']])


@then('has a exclamation mark before the scope violation is detected.')
def assert_exclamation_mark_before_scope_violation(context):
    # When/Then
    result = assert_linting_fails(context)

    # Then
    assert_commits_linting_errors(
        result, [['NonConventionalCommitsSpecification', 'ExclamationMarkBeforeScope']])


@then('has a scope which is empty violation is detected.')
def assert_empty_scope_violation(context):
    # When/Then
    result = assert_linting_fails(context)

    # Then
    assert_commits_linting_errors(
        result, [['NonConventionalCommitsSpecification', 'EmptyScope']])


@then('has no space after the colon preceding the type and scope violation is detected.')
def assert_no_space_after_type_violation(context):
    # When/Then
    result = assert_linting_fails(context)

    # Then
    assert_commits_linting_errors(result,
                                  [['NonConventionalCommitsSpecification',
                                    'NoSpaceAfterColonPrecedingTypeAndScope']])


@then('has no description after the type and scope violation is detected.')
def assert_no_description_violation(context):
    # When/Then
    result = assert_linting_fails(context)

    # Then
    assert_commits_linting_errors(result,
                                  [['NonConventionalCommitsSpecification',
                                    'NoDescriptionAfterTypeAndScope']])


@then('a commit title too long violation is detected.')
def assert_commit_title_too_long_violation(context):
    # When/Then
    result = assert_linting_fails(context)

    # Then
    assert_commits_linting_errors(result, [['CommitTitleTooLong']])
