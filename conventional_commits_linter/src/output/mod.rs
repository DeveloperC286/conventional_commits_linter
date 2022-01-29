use strum_macros::EnumString;

#[allow(clippy::upper_case_acronyms)]
#[derive(Debug, PartialEq, Eq, EnumString)]
pub enum Output {
    Quiet,
    Pretty,
    JSON,
}
