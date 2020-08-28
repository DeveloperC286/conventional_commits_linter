use super::*;

pub fn is_position_in_binary_string_true(binary_string: &String, position: usize) -> bool {
    match binary_string.chars().nth(position).unwrap() {
        '0' => false,
        '1' => true,
        _ => {
            panic!("Should be either 0 or 1.");
        }
    }
}

//NO_SPACE_AFTER_TYPE,
pub fn generate_commit(
    should_generate_preceding_whitespace: bool,
    should_generate_empty_scope: bool,
    should_generate_no_description: bool,
) -> (Vec<Commit>, Vec<LintingError>) {
    let mut linting_errors = vec![LintingError::NON_CONVENTIONAL_COMMITS_SPECIFICATION];
    let mut commits: Vec<Commit> = vec![];

    let scope_variations = match should_generate_empty_scope {
        true => {
            linting_errors.push(LintingError::EMPTY_SCOPE);
            get_empty_scope_variations()
        }
        false => vec!["".to_string()],
    };

    let preceding_variations = match should_generate_preceding_whitespace {
        true => {
            linting_errors.push(LintingError::PRECEDING_WHITESPACE);
            get_preceding_whitespace_variations()
        }
        false => vec!["".to_string()],
    };

    let description_variations = match should_generate_no_description {
        true => {
            linting_errors.push(LintingError::NO_DESCRIPTION);
            get_no_description_variations()
        }
        false => get_description_variations(),
    };

    let mut commit_id = 1;
    for description in description_variations {
        for preceding in &preceding_variations {
            for scope in &scope_variations {
                for commit_type in get_commit_type_variations() {
                    let generated_commit =
                        format!("{}{}{}: {}", preceding, commit_type, scope, description);
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

    return (commits, linting_errors);
}

fn get_no_description_variations() -> Vec<String> {
    return vec![
        "".to_string(),
        "\t".to_string(),
        "\n\n".to_string(),
        "\n\r".to_string(),
    ];
}

fn get_description_variations() -> Vec<String> {
    return vec![
        "this is a description".to_string(),
        "this is a description\n\n".to_string(),
    ];
}

fn get_preceding_whitespace_variations() -> Vec<String> {
    return vec![
        "  ".to_string(),
        " ".to_string(),
        "\t".to_string(),
        "\n\r".to_string(),
    ];
}

fn get_empty_scope_variations() -> Vec<String> {
    return vec![
        "()".to_string(),
        "()!".to_string(),
        "!()".to_string(),
        "( )".to_string(),
    ];
}

fn get_commit_type_variations() -> Vec<String> {
    return vec![
        "bug".to_string(),
        "fix".to_string(),
        "feat".to_string(),
        "ci".to_string(),
        "chore".to_string(),
        "docs".to_string(),
    ];
}
