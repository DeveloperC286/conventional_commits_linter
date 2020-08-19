use crate::model::LintingError;
use console::Style;
use git2::Oid;
use std::collections::HashMap;

pub fn print_summary(linting_errors: &HashMap<Oid, Vec<LintingError>>) {
    let number_of_issues: usize = linting_errors.values().map(|x| x.len()).sum();
    let red = Style::new().red().bold();

    println!(
        "{} - Found {} seperate linting issues across {} commits.",
        red.apply_to("X"),
        number_of_issues,
        linting_errors.len()
    );
}
