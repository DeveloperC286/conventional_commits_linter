import json
from behave import *

from utilities import is_json
from then import then_linting_fails


@then('their are "{number_of_commits}" commits failing linting.')
def then_number_of_commits_failing_linting(context, number_of_commits):
    # When/Then
    then_linting_fails(context)

    # Then
    assert is_json(context.stdout)
    output = json.loads(context.stdout)
    assert len(output) == int(number_of_commits)


@then('has preceding whitespace characters violation is detected.')
def then_preceding_whitespace_violation(context):
    # When/Then
    then_linting_fails(context)

    # Then
    assert is_json(context.stdout)
    output = json.loads(context.stdout)
    assert len(output) == 1
    linting_errors = output[0]['linting_errors']
    assert len(linting_errors) == 2
    assert 'NonConventionalCommitsSpecification' in linting_errors
    assert 'PrecedingWhitespace' in linting_errors


@then('a non-Angular type violation is detected.')
def then_non_angular_type_violation(context):
    # When/Then
    then_linting_fails(context)

    # Then
    assert is_json(context.stdout)
    output = json.loads(context.stdout)
    assert len(output) == 1
    linting_errors = output[0]['linting_errors']
    assert len(linting_errors) == 1
    assert 'NonAngularType' in linting_errors


@then('has a exclamation mark before the scope violation is detected.')
def then_exclamation_mark_before_scope_violation(context):
    # When/Then
    then_linting_fails(context)

    # Then
    assert is_json(context.stdout)
    output = json.loads(context.stdout)
    assert len(output) == 1
    linting_errors = output[0]['linting_errors']
    assert len(linting_errors) == 2
    assert 'NonConventionalCommitsSpecification' in linting_errors
    assert 'ExclamationMarkBeforeScope' in linting_errors


@then('has a scope which is empty violation is detected.')
def then_empty_scope_violation(context):
    # When/Then
    then_linting_fails(context)

    # Then
    assert is_json(context.stdout)
    output = json.loads(context.stdout)
    assert len(output) == 1
    linting_errors = output[0]['linting_errors']
    assert len(linting_errors) == 2
    assert 'NonConventionalCommitsSpecification' in linting_errors
    assert 'EmptyScope' in linting_errors


@then('has no space after the colon preceding the type and scope violation is detected.')
def then_no_space_after_type_violation(context):
    # When/Then
    then_linting_fails(context)

    # Then
    assert is_json(context.stdout)
    output = json.loads(context.stdout)
    assert len(output) == 1
    linting_errors = output[0]['linting_errors']
    assert len(linting_errors) == 2
    assert 'NonConventionalCommitsSpecification' in linting_errors
    assert 'NoSpaceAfterColonPrecedingTypeAndScope' in linting_errors


@then('has no description after the type and scope violation is detected.')
def then_no_description_violation(context):
    # When/Then
    then_linting_fails(context)

    # Then
    assert is_json(context.stdout)
    output = json.loads(context.stdout)
    assert len(output) == 1
    linting_errors = output[0]['linting_errors']
    assert len(linting_errors) == 2
    assert 'NonConventionalCommitsSpecification' in linting_errors
    assert 'NoDescriptionAfterTypeAndScope' in linting_errors
