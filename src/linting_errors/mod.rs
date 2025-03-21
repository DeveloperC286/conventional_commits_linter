use std::collections::{HashMap, VecDeque};

use anyhow::Result;

use crate::commits::commit::Commit;
use crate::linting_error::LintingError;
use crate::source::Source;

mod json;
mod pretty;

/// A representation of the linting errors within the range of commits.
pub struct LintingErrors {
    source: Source,
    order: VecDeque<Commit>,
    errors: HashMap<Commit, Vec<LintingError>>,
}

impl LintingErrors {
    pub(crate) fn from(
        source: Source,
        order: VecDeque<Commit>,
        errors: HashMap<Commit, Vec<LintingError>>,
    ) -> LintingErrors {
        LintingErrors {
            source,
            order,
            errors,
        }
    }

    /// Get a pretty representation of the linting errors as a string, it is suitable as output for
    /// a user.
    pub fn pretty(&self) -> String {
        pretty::print_all(self.source, &self.order, &self.errors)
    }

    /// Get a JSON representation of the linting errors as a string, it is suitable as output for
    /// machine interpretation.
    pub fn json(&self) -> Result<String> {
        json::print_all(&self.order, &self.errors)
    }
}
