use regex::Regex;

pub fn lint(commit_message: &str) -> bool {
    lazy_static! {
        static ref ANGULAR_TYPE_REGEX: Regex = Regex::new(
            r"(?i)^(revert|build|ci|docs|feat|fix|perf|refactor|style|test)(\([[:alpha:]]*\))?(!)?:"
        )
        .unwrap();
    }

    match ANGULAR_TYPE_REGEX.is_match(commit_message) {
        true => true,
        false => {
            error!("{:?} uses a non Angular type.", commit_message);
            false
        }
    }
}

#[cfg(test)]
mod tests;
