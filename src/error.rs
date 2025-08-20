#[derive(Debug)]
pub enum Err {
    Misc(String),
}

impl std::fmt::Display for Err {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Err::Misc(s) => write!(f, "{}", s),
        }
    }
}

impl std::error::Error for Err {}

pub type Result<T> = std::result::Result<T, Err>;
