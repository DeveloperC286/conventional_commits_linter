use super::*;

mod variations;

pub(super) fn generate_non_angular_type_commits() -> Vec<(Vec<String>, Vec<LintingError>)> {
    generate_commit_messages(variations::NON_ANGULAR_COMMIT_TYPE_VARIATIONS)
}

pub(super) fn generate_angular_type_commits() -> Vec<(Vec<String>, Vec<LintingError>)> {
    generate_commit_messages(variations::ANGULAR_COMMIT_TYPE_VARIATIONS)
}

fn generate_commit_messages(
    commit_type_variations: &[&str],
) -> Vec<(Vec<String>, Vec<LintingError>)> {
    let mut commit_messages = vec![];

    for (preceding_whitespace_variations_errors, preceding_whitespace_variations) in
        variations::PRECEDING_VARIATIONS
    {
        for (scope_variations_errors, scope_variations) in variations::SCOPE_VARIATIONS {
            for (after_type_variations_errors, after_type_variations) in
                variations::AFTER_TYPE_VARIATIONS
            {
                for (
                    description_and_body_variations_errors,
                    description_variations,
                    body_variations,
                ) in variations::DESCRIPTION_AND_BODY_VARIATIONS
                {
                    let mut linting_errors = vec![];
                    linting_errors.extend_from_slice(preceding_whitespace_variations_errors);
                    linting_errors.extend_from_slice(scope_variations_errors);
                    linting_errors.extend_from_slice(after_type_variations_errors);
                    linting_errors.extend_from_slice(description_and_body_variations_errors);

                    if !linting_errors.is_empty() {
                        linting_errors.push(LintingError::NonConventionalCommitsSpecification);
                    }

                    commit_messages.push((
                        generate_commit_messages_from(
                            preceding_whitespace_variations,
                            commit_type_variations,
                            scope_variations,
                            after_type_variations,
                            description_variations,
                            body_variations,
                        ),
                        linting_errors,
                    ));
                }
            }
        }
    }

    commit_messages
}

fn generate_commit_messages_from(
    preceding_whitespace_variations: &[&str],
    commit_type_variations: &[&str],
    scope_variations: &[&str],
    after_type_variations: &[&str],
    description_variations: &[&str],
    body_variations: &[&str],
) -> Vec<String> {
    let mut commit_messages = vec![];

    for preceding_whitespace in preceding_whitespace_variations {
        for commit_type in commit_type_variations {
            for scope in scope_variations {
                for after_type in after_type_variations {
                    for description in description_variations {
                        for body in body_variations {
                            commit_messages.push(format!(
                            "{preceding_whitespace}{commit_type}{scope}:{after_type}{description}{body}"
                        ));
                        }
                    }
                }
            }
        }
    }

    commit_messages
}
