import os
import json
from subprocess import Popen, PIPE


def execute_command(command):
    process = Popen(
        command,
        shell=True,
        stdin=PIPE,
        stdout=PIPE,
        stderr=PIPE)
    process.wait()

    stdout, stderr = process.communicate()
    return process.returncode, stdout.decode("utf-8"), stderr.decode("utf-8")


def execute_conventional_commits_linter(context):
    if "GIT_DIR" not in os.environ:
        os.chdir(context.remote_repository_cache)

    (context.exit_code, context.stdout, context.stderr) = execute_command(
        context.pre_command + context.conventional_commits_linter_path + context.arguments)

    if "GIT_DIR" not in os.environ:
        os.chdir(context.behave_directory)


def is_json(testing):
    try:
        json.loads(testing)
    except ValueError as _:
        return False
    return True