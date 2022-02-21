from behave import *


@when(
    'the flag --from-stdin is set and the standard input is "{standard_input}".')
def set_from_stdin(context, standard_input):
    context.standard_input = standard_input.strip()
    context.pre_command = "echo " + context.standard_input + " | "
    context.arguments += " --from-stdin "


@when('the argument --from-reference is provided as "{from_reference}".')
def set_from_reference(context, from_reference):
    context.arguments += " --from-reference " + from_reference + " "


@when('the argument --from-commit-hash is provided as "{from_commit_hash}".')
def set_from_commit_hash(context, from_commit_hash):
    context.arguments += " --from-commit-hash " + from_commit_hash + " "


@when('the argument --git-history-mode is provided as "AllParents".')
def set_batch_commits_flag(context):
    context.arguments += " --git-history-mode \"AllParents\" "


@when('the flag --allow-angular-type-only is set.')
def set_allow_angular_type_only(context):
    context.arguments += " --allow-angular-type-only "


@when('the argument --output is set as "Quiet".')
def set_quiet(context):
    context.arguments += " --output \"Quiet\" "


@when('the argument --output is set as "JSON".')
def set_json(context):
    context.arguments += " --output \"JSON\" "
