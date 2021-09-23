import os


def after_scenario(context, _):
    os.chdir(context.behave_directory)
