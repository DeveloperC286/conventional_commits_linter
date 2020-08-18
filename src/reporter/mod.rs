use crate::model::LintingError;
use git2::Oid;
use std::collections::HashMap;

pub fn print_summary(linting_errors: &HashMap<Oid, Vec<LintingError>>) {
    let number_of_issues: usize = linting_errors.values().map(|x| x.len()).sum();

    println!(
        "Found {} issues across {} commits.",
        number_of_issues,
        linting_errors.len()
    );
}
