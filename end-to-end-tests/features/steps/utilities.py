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

    result = type("Result", (), {})
    result.exit_code = process.returncode

    stdout, stderr = process.communicate()
    result.stdout = stdout.decode("utf-8")
    result.stderr = stderr.decode("utf-8")

    return result


def execute_conventional_commits_linter(context):
    if "GIT_DIR" not in os.environ:
        os.chdir(context.remote_repository_cache)

    result = execute_command(
        context.pre_command +
        context.conventional_commits_linter_path +
        context.arguments)

    if "GIT_DIR" not in os.environ:
        os.chdir(context.behave_directory)

    return result
