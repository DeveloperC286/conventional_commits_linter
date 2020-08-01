use regex::Regex;

pub fn lint_commits(commit_messages: Vec<String>) -> usize {
    let mut number_of_linting_errors = 0;

    for commit_message in commit_messages {
        if !lint_commit(&commit_message) {
            number_of_linting_errors += 1;
        }
    }

    number_of_linting_errors
}

pub fn lint_commit(commit_message: &str) -> bool {
    lazy_static! {
        static ref CONVENTIONAL_COMMITS_REGEX: Regex =
            Regex::new(r"^([[:alpha:]])+(\([[:alpha:]]*\))?(!)?: (.)+").unwrap();
    }

    match CONVENTIONAL_COMMITS_REGEX.is_match(commit_message) {
        true => true,
        false => {
            error!(
                "{:?} does not match the Conventional Commits v1.0.0 format.",
                commit_message
            );
            false
        }
    }
}

#[cfg(test)]
mod tests;
