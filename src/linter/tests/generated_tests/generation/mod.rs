use super::*;

mod variations;

pub fn generate_commit(
    should_generate_preceding_whitespace: bool,
    should_generate_empty_scope: bool,
    should_generate_no_description: bool,
    should_generate_no_space_after_type: bool,
) -> (Vec<Commit>, Vec<LintingError>) {
    let mut linting_errors = vec![LintingError::NonConventionalCommitsSpecification];
    let mut commits: Vec<Commit> = vec![];

    let scope_variations = match should_generate_empty_scope {
        true => {
            linting_errors.push(LintingError::EmptyScope);
            variations::get_empty_scope_variations()
        }
        false => vec!["".to_string()],
    };

    let preceding_variations = match should_generate_preceding_whitespace {
        true => {
            linting_errors.push(LintingError::PrecedingWhitespace);
            variations::get_preceding_whitespace_variations()
        }
        false => vec!["".to_string()],
    };

    let description_variations = match should_generate_no_description {
        true => {
            linting_errors.push(LintingError::NoDescription);
            variations::get_no_description_variations()
        }
        false => variations::get_description_variations(),
    };

    let space_after_type = match should_generate_no_space_after_type {
        true => {
            linting_errors.push(LintingError::NoSpaceAfterType);
            ""
        }
        false => " ",
    };

    let mut commit_id = 1;
    for description in description_variations {
        for preceding in &preceding_variations {
            for scope in &scope_variations {
                for commit_type in variations::get_commit_type_variations() {
                    let generated_commit = format!(
                        "{}{}{}:{}{}",
                        preceding, commit_type, scope, space_after_type, description
                    );
                    let commit = Commit {
                        oid: git2::Oid::from_str(&commit_id.to_string()).unwrap(),
                        message: generated_commit.to_string(),
                    };
                    commits.push(commit);
                    commit_id += 1;
                }
            }
        }
    }

    (commits, linting_errors)
}
