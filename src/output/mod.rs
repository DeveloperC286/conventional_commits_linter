#[allow(clippy::upper_case_acronyms)]
#[derive(Debug, PartialEq, Eq, Clone, strum_macros::EnumString)]
#[strum(ascii_case_insensitive)]
pub enum Output {
    Quiet,
    Pretty,
    JSON,
}
