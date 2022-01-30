import os
import hashlib
from behave import given

from utilities import execute_command


@given('the context and environment are reset.')
def reset_context(context):
    context.behave_directory = os.getcwd()
    context.remote_repository_cache = os.getcwd()

    context.pre_command = ""
    context.conventional_commits_linter_path = context.behave_directory + \
        "/../target/debug/conventional_commits_linter"
    context.arguments = ""

    if "GIT_DIR" in os.environ:
        del os.environ["GIT_DIR"]


@given('the repository "{remote_repository}" is cloned and checked out at the commit "{commit_hash}".')
def clone_remote_repository_and_checkout_commit(
        context, remote_repository, commit_hash):
    reset_context(context)

    remote_repository_md5 = hashlib.md5(remote_repository.encode())
    context.remote_repository_cache = "/tmp/" + remote_repository_md5.hexdigest()

    if not os.path.exists(context.remote_repository_cache):
        (exit_code, _, _) = execute_command("git clone " +
                                            remote_repository + " " + context.remote_repository_cache)
        assert exit_code == 0

    os.chdir(context.remote_repository_cache)

    (exit_code, _, _) = execute_command("git reset --hard origin/HEAD")
    assert exit_code == 0

    (exit_code, _, _) = execute_command("git clean -fdx")
    assert exit_code == 0

    (exit_code, _, _) = execute_command("git checkout " + commit_hash)
    assert exit_code == 0

    os.chdir(context.behave_directory)


@given('the GIT_DIR environment variable is set to the cloned repository.')
def set_git_dir(context):
    os.environ["GIT_DIR"] = str(context.remote_repository_cache + "/.git")
