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
    let number_of_variants: usize = 7;

    for i in 1..2_usize.pow(number_of_variants as u32) {
        let binary_string = format!("{:0number_of_variants$b}", i);

        let mut linting_errors = vec![LintingError::NonConventionalCommitsSpecification];

        let should_generate_preceding_whitespace_variations =
            is_position_in_binary_string_true(&binary_string, 0);
        let should_generate_scope_variations = is_position_in_binary_string_true(&binary_string, 1);
        let should_generate_incorrect_breaking_change_in_title =
            is_position_in_binary_string_true(&binary_string, 2);
        let should_generate_after_type_variation =
            is_position_in_binary_string_true(&binary_string, 3);
        let should_generate_description_variations =
            is_position_in_binary_string_true(&binary_string, 4);
        let should_generate_description_termination =
            is_position_in_binary_string_true(&binary_string, 5);
        let should_generate_body = is_position_in_binary_string_true(&binary_string, 6);

        let preceding_whitespace_variations = variations::get_preceding_whitespace_variations(
            &mut linting_errors,
            should_generate_preceding_whitespace_variations,
        );
        let scope_variations =
            variations::get_scope_variations(&mut linting_errors, should_generate_scope_variations);
        let scope_variations = variations::get_incorrect_breaking_change_title_variations(
            &mut linting_errors,
            scope_variations,
            should_generate_incorrect_breaking_change_in_title,
        );
        let after_type_variation = variations::get_after_type_variation(
            &mut linting_errors,
            should_generate_after_type_variation,
        );
        let description_variations = variations::get_description_variations(
            &mut linting_errors,
            should_generate_description_variations,
        );
        let description_termination_variations = variations::get_description_termination_variations(
            should_generate_description_termination || should_generate_body,
        );
        let body_variations = variations::get_body_variations(should_generate_body);

        // As only these variations produce linting errors, the body variations do not.
        if should_generate_preceding_whitespace_variations
            || should_generate_scope_variations
            || should_generate_incorrect_breaking_change_in_title
            || should_generate_after_type_variation
            || should_generate_description_variations
        {
            commit_messages.push((
                generate_commit_messages_from(
                    preceding_whitespace_variations,
                    commit_type_variations,
                    &scope_variations,
                    after_type_variation,
                    description_variations,
                    description_termination_variations,
                    body_variations,
                ),
                linting_errors,
            ));
        }
    }

    commit_messages
}

fn generate_commit_messages_from(
    preceding_whitespace_variations: &[&str],
    commit_type_variations: &[&str],
    scope_variations: &[String],
    after_type_variation: &str,
    description_variations: &[&str],
    description_termination_variations: &[&str],
    body_variations: &[&str],
) -> Vec<String> {
    let mut commit_messages = vec![];

    for preceding_whitespace in preceding_whitespace_variations {
        for commit_type in commit_type_variations {
            for scope in scope_variations {
                for description in description_variations {
                    for description_termination in description_termination_variations {
                        for body in body_variations {
                            commit_messages.push(format!("{preceding_whitespace}{commit_type}{scope}:{after_type_variation}{description}{description_termination}{body}"));
                        }
                    }
                }
            }
        }
    }

    commit_messages
}

fn is_position_in_binary_string_true(binary_string: &str, position: usize) -> bool {
    match binary_string.chars().nth(position).unwrap() {
        '0' => false,
        '1' => true,
        _ => {
            panic!("Should be either 0 or 1.");
        }
    }
}
