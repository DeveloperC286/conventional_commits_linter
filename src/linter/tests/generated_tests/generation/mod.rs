use super::*;

mod variations;

pub(crate) fn generate_non_angular_type_commits(
    should_generate_preceding_whitespace: bool,
    should_generate_empty_scope: bool,
    should_not_generate_space_after_type: bool,
    should_not_generate_description: bool,
) -> (Vec<String>, Vec<LintingError>) {
    generate_commits_with_type(
        should_generate_preceding_whitespace,
        variations::NON_ANGULAR_COMMIT_TYPE_VARIATIONS,
        should_generate_empty_scope,
        should_not_generate_space_after_type,
        should_not_generate_description,
    )
}

pub(crate) fn generate_angular_type_commits(
    should_generate_preceding_whitespace: bool,
    should_generate_empty_scope: bool,
    should_not_generate_space_after_type: bool,
    should_not_generate_description: bool,
) -> (Vec<String>, Vec<LintingError>) {
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
) -> (Vec<String>, Vec<LintingError>) {
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
        generate_commit_messages(
            preceding_whitespace_variations,
            commit_type_variations,
            scope_variations,
            after_type_variation,
            description_variations,
        ),
        linting_errors,
    )
}

fn generate_commit_messages(
    preceding_whitespace_variations: &[&str],
    commit_type_variations: &[&str],
    scope_variations: &[&str],
    after_type_variation: &str,
    description_variations: &[&str],
) -> Vec<String> {
    let mut commit_messages = vec![];

    for preceding_whitespace in preceding_whitespace_variations {
        for commit_type in commit_type_variations {
            for scope in scope_variations {
                for description in description_variations {
                    commit_messages.push(format!(
                        "{}{}{}:{}{}",
                        preceding_whitespace, commit_type, scope, after_type_variation, description
                    ));
                }
            }
        }
    }

    commit_messages
}
