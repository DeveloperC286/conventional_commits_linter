#![doc = include_str!("../README.md")]
#![deny(missing_docs)]
// Disabling because of rstest_reuse.
#![allow(clippy::single_component_path_imports)]

#[cfg(test)]
use rstest_reuse;

#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate log;
extern crate regex;

mod commits;
mod git_history_mode;
mod linting_error;
mod linting_errors;
mod source;

pub use crate::commits::Commits;
pub use crate::git_history_mode::GitHistoryMode;
pub use crate::linting_error::LintingError;
pub use crate::linting_errors::LintingErrors;
