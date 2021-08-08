use super::*;

mod variations;

pub fn generate_non_angular_type_commits(
    should_generate_preceding_whitespace: bool,
    should_generate_empty_scope: bool,
    should_not_generate_space_after_type: bool,
    should_not_generate_description: bool,
) -> (Vec<Commit>, Vec<LintingError>) {
    generate_commits_with_type(
        should_generate_preceding_whitespace,
        variations::NON_ANGULAR_COMMIT_TYPE_VARIATIONS,
        should_generate_empty_scope,
        should_not_generate_space_after_type,
        should_not_generate_description,
    )
}

pub fn generate_angular_type_commits(
    should_generate_preceding_whitespace: bool,
    should_generate_empty_scope: bool,
    should_not_generate_space_after_type: bool,
    should_not_generate_description: bool,
) -> (Vec<Commit>, Vec<LintingError>) {
    generate_commits_with_type(
        should_generate_preceding_whitespace,
        variations::ANGULAR_COMMIT_TYPE_VARIATIONS,
        should_generate_empty_scope,
        should_not_generate_space_after_type,
        should_not_generate_description,
    )
}

fn generate_commits_with_type(
    should_generate_preceding_whitespace: bool,
    commit_type_variations: &[&str],
    should_generate_empty_scope: bool,
    should_not_generate_space_after_type: bool,
    should_not_generate_description: bool,
) -> (Vec<Commit>, Vec<LintingError>) {
    let mut linting_errors = vec![LintingError::NonConventionalCommitsSpecification];

    let preceding_whitespace_variations = variations::get_preceding_whitespace_variations(
        &mut linting_errors,
        should_generate_preceding_whitespace,
    );
    let scope_variations =
        variations::get_scope_variations(&mut linting_errors, should_generate_empty_scope);
    let after_type_variation = variations::get_after_type_variation(
        &mut linting_errors,
        should_not_generate_space_after_type,
    );
    let description_variations = variations::get_description_variations(
        &mut linting_errors,
        should_not_generate_description,
    );

    (
        generate_commits(
            preceding_whitespace_variations,
            commit_type_variations,
            scope_variations,
            after_type_variation,
            description_variations,
        ),
        linting_errors,
    )
}

fn generate_commits(
    preceding_whitespace_variations: &[&str],
    commit_type_variations: &[&str],
    scope_variations: &[&str],
    after_type_variation: &str,
    description_variations: &[&str],
) -> Vec<Commit> {
    let mut commits: Vec<Commit> = vec![];
    let mut commit_id = 1;

    for preceding_whitespace in preceding_whitespace_variations {
        for commit_type in commit_type_variations {
            for scope in scope_variations {
                for description in description_variations {
                    commits.push(Commit {
                        oid: git2::Oid::from_str(&commit_id.to_string()).unwrap(),
                        message: format!(
                            "{}{}{}:{}{}",
                            preceding_whitespace,
                            commit_type,
                            scope,
                            after_type_variation,
                            description
                        ),
                    });
                    commit_id += 1;
                }
            }
        }
    }

    commits
}
