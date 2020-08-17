use regex::Regex;

pub fn lint(commit_message: &str) -> bool {
    lazy_static! {
        static ref CONVENTIONAL_COMMITS_REGEX: Regex =
            Regex::new(r"^([[:alpha:]])+(\([[:alpha:]]+\))?(!)?: (.)+").unwrap();
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
