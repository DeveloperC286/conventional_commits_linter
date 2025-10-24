import tempfile
from behave import *


@when('linting the "{commit_message}".')
def set_linting_the(context, commit_message):
    context.commit_message = commit_message.strip()
    context.pre_command = f"echo -e {context.commit_message} | "
    context.from_ref = " \"-\""
    # Testing we use stdin when not in a Git repository.
    # https://gitlab.com/DeveloperC/conventional_commits_linter/-/issues/3
    context.remote_repository_cache = tempfile.mkdtemp()


@when('linting from the "{git}".')
def set_linting_from_the(context, git):
    context.from_ref = f" \"{git}\""


@when('the argument --history-mode is provided as "{history_mode}".')
def set_history_mode(context, history_mode):
    context.arguments += f" --history-mode {history_mode} "


@when('the flag --type is set as "{type}".')
def set_type(context, type):
    context.arguments += f" --type {type} "


@when('the argument --output is set as "{output}".')
def set_output(context, output):
    context.arguments += f" --output {output} "


@when('the argument --max-commit-title-length is set to "{max_length}".')
def set_max_commit_title_length(context, max_length):
    context.arguments += f" --max-commit-title-length {max_length} "


@when('the argument --lowercase-scope is set.')
def set_lowercase_scope(context):
    context.arguments += " --lowercase-scope "
