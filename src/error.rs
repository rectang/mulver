use std::fmt;

#[cfg(test)]
mod tests;

/// Mulver-specific Result type.
pub type Result<T> = std::result::Result<T, MulverError>;

/// An error which occurs inside Mulver.
#[derive(Debug)]
pub enum MulverError {
    /// Error of unspecified or unknown type.
    Unspecified,
}

impl fmt::Display for MulverError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            MulverError::Unspecified => write!(f, "Unspecified Mulver Error"),
        }
    }
}
