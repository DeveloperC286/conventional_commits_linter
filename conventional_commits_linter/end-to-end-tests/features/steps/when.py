import tempfile
from behave import *


@when(
    'the flag --from-stdin is set and the standard input is "{standard_input}".')
def set_from_stdin(context, standard_input):
    context.standard_input = standard_input.strip()
    context.pre_command = f"echo {context.standard_input} | "
    context.arguments += " --from-stdin "
    # Testing we use stdin when not in a Git repository.
    # https://gitlab.com/DeveloperC/conventional_commits_linter/-/issues/3
    context.remote_repository_cache = tempfile.mkdtemp()


@when('the argument --from-reference is provided as "{from_reference}".')
def set_from_reference(context, from_reference):
    context.arguments += f" --from-reference {from_reference} "


@when('the argument --from-commit-hash is provided as "{from_commit_hash}".')
def set_from_commit_hash(context, from_commit_hash):
    context.arguments += f" --from-commit-hash {from_commit_hash} "


@when('the argument --git-history-mode is provided as "{git_history_mode}".')
def set_batch_commits_flag(context, git_history_mode):
    context.arguments += f" --git-history-mode {git_history_mode} "


@when('the flag --allow-angular-type-only is set.')
def set_allow_angular_type_only(context):
    context.arguments += " --allow-angular-type-only "


@when('the argument --output is set as "{output}".')
def set_output(context, output):
    context.arguments += f" --output {output} "
