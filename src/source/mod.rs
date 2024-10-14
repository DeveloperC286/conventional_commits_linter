#[derive(Clone, Copy, PartialEq, Eq)]
pub(crate) enum Source {
    Git,
    CommitMessage,
}
