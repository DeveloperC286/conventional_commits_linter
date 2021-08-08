import json
from behave import *

from utilities import is_json


@then('a preceding whitespace before the type violation is detected.')
def then_preceding_whitespace_before_type_violation(context):
    assert is_json(context.stdout)
    output = json.loads(context.stdout)
    assert len(output) == 1
    linting_errors = output[0]['linting_errors']
    assert len(linting_errors) == 2
    assert 'NonConventionalCommitsSpecification' in linting_errors
    assert 'PrecedingWhitespace' in linting_errors


@then('a non-Angular type violation is detected.')
def then_non_angular_type_violation(context):
    assert is_json(context.stdout)
    output = json.loads(context.stdout)
    assert len(output) == 1
    linting_errors = output[0]['linting_errors']
    assert len(linting_errors) == 1
    assert 'NonAngularType' in linting_errors


@then('a empty scope violation is detected.')
def then_empty_scope_violation(context):
    assert is_json(context.stdout)
    output = json.loads(context.stdout)
    assert len(output) == 1
    linting_errors = output[0]['linting_errors']
    assert len(linting_errors) == 2
    assert 'NonConventionalCommitsSpecification' in linting_errors
    assert 'EmptyScope' in linting_errors


@then('a no space after the colon preceding the type and scope violation is detected.')
def then_no_space_after_type_violation(context):
    assert is_json(context.stdout)
    output = json.loads(context.stdout)
    assert len(output) == 1
    linting_errors = output[0]['linting_errors']
    assert len(linting_errors) == 2
    assert 'NonConventionalCommitsSpecification' in linting_errors
    assert 'NoSpaceAfterColonPrecedingTypeAndScope' in linting_errors


@then('a no description after the type and scope violation is detected.')
def then_no_description_violation(context):
    assert is_json(context.stdout)
    output = json.loads(context.stdout)
    assert len(output) == 1
    linting_errors = output[0]['linting_errors']
    assert len(linting_errors) == 2
    assert 'NonConventionalCommitsSpecification' in linting_errors
    assert 'NoDescriptionAfterTypeAndScope' in linting_errors
