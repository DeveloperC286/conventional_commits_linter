from behave import *


@then('a preceding whitespace before the type violation is detected.')
def then_preceding_whitespace_before_type_violation(context):
    preceding_whitespace_before_the_type = "Message - \"" + \
                                           context.standard_input.strip('"') + \
                                           "\\n\"\n\tX - Commit title does not comply with the Conventional Commits V1.0.0 specification.\n\tX - Commit title has preceding whitespace characters.\n\n"
    assert context.stdout == preceding_whitespace_before_the_type


@then('a non-Angular type violation is detected.')
def then_non_angular_type_violation(context):
    non_angular_type = "Message - \"" + \
        context.standard_input.strip('"') + \
        "\\n\"\n\tX - Commit title does not use an Angular type.\n\n"
    assert context.stdout == non_angular_type


@then('a empty scope violation is detected.')
def then_empty_scope_violation(context):
    empty_scope = "Message - \"" + \
                  context.standard_input.strip('"') + \
                  "\\n\"\n\tX - Commit title does not comply with the Conventional Commits V1.0.0 specification.\n\tX - Commit title has a scope which is empty.\n\n"
    assert context.stdout == empty_scope


@then('a no space after the colon preceding the type and scope violation is detected.')
def then_no_space_after_type_violation(context):
    no_space_after_type = "Message - \"" + \
                          context.standard_input.strip('"') + \
                          "\\n\"\n\tX - Commit title does not comply with the Conventional Commits V1.0.0 specification.\n\tX - Commit title has no space after the colon preceding the Conventional Commits type and scope.\n\n"
    assert context.stdout == no_space_after_type


@then('a no description after the type and scope violation is detected.')
def then_no_description_violation(context):
    no_description = "Message - \"" + \
                     context.standard_input.strip('"') + \
                     "\\n\"\n\tX - Commit title does not comply with the Conventional Commits V1.0.0 specification.\n\tX - Commit title has no description after the Conventional Commits type and scope.\n\n"
    assert context.stdout == no_description
