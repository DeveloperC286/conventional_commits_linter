pub fn get_no_description_variations() -> Vec<String> {
    return vec![
        "".to_string(),
        "\t".to_string(),
        "\n\n".to_string(),
        "\n\r".to_string(),
    ];
}

pub fn get_description_variations() -> Vec<String> {
    return vec![
        "this is a description".to_string(),
        "this is a description\n\n".to_string(),
    ];
}

pub fn get_preceding_whitespace_variations() -> Vec<String> {
    return vec![
        "  ".to_string(),
        " ".to_string(),
        "\t".to_string(),
        "\n\r".to_string(),
    ];
}

pub fn get_empty_scope_variations() -> Vec<String> {
    return vec![
        "()".to_string(),
        "()!".to_string(),
        "!()".to_string(),
        "( )".to_string(),
    ];
}

pub fn get_commit_type_variations() -> Vec<String> {
    return vec![
        "bug".to_string(),
        "fix".to_string(),
        "feat".to_string(),
        "ci".to_string(),
        "chore".to_string(),
        "docs".to_string(),
    ];
}
