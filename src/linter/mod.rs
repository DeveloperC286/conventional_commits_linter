mod allow_angular_type_only;
mod conventional_commits_specification;

pub fn lint_commits(commit_messages: Vec<String>, allow_angular_type_only: bool) -> usize {
    let mut number_of_linting_errors = 0;

    for commit_message in commit_messages {
        if !conventional_commits_specification::lint(&commit_message) {
            number_of_linting_errors += 1;
        }

        if allow_angular_type_only && !allow_angular_type_only::lint(&commit_message) {
            number_of_linting_errors += 1;
        }
    }

    number_of_linting_errors
}
