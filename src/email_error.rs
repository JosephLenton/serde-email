use ::std::error::Error;
use ::std::fmt::Display;
use ::std::fmt::Formatter;
use ::std::fmt::Result as FmtResult;

#[derive(Clone, Debug, PartialEq)]
pub enum EmailError {
    Invalid { raw_email: String },
}

impl Error for EmailError {}

impl Display for EmailError {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        match self {
            EmailError::Invalid { raw_email } => {
                write!(f, "invalid email address, was given '{}'", raw_email)
            }
        }
    }
}
