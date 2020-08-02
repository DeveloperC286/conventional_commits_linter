from subprocess import Popen, PIPE, STDOUT


def execute_command(command):
    process = Popen(
        command,
        shell=True,
        stdin=PIPE,
        stdout=PIPE,
        stderr=STDOUT)
    process.wait()

    return (process.returncode, process.stdout.read().decode('utf-8'))
