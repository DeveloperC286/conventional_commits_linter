use strum_macros::{Display, EnumString};

/// The mode to use when transversing the Git commit history of the Git commit range, to collect
/// the Git commit messages to use in calculating the next semantic version.
#[derive(Debug, PartialEq, Eq, Clone, Copy, Display, EnumString)]
pub enum GitHistoryMode {
    /// In FirstParent mode only the first parent of merge commit's are parsed for their Git commit
    /// messages.
    FirstParent,
    /// In AllParents mode all the parents of merge commit's are parsed for their Git commit
    /// messages.
    AllParents,
}
