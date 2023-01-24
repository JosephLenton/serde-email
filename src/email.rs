use ::std::convert::AsRef;
use ::std::convert::Into;
use ::std::fmt::Debug;
use ::std::fmt::Display;
use ::std::fmt::Formatter;
use ::std::fmt::Result as FmtResult;

use crate::is_valid_email;
use crate::EmailError;

#[cfg(feature = "serde")]
mod email_visitor;
#[cfg(feature = "serde")]
pub(crate) use self::email_visitor::*;

#[cfg(feature = "serde")]
mod serde_support;
#[cfg(feature = "serde")]
pub use self::serde_support::*;

#[cfg(feature = "sea-orm")]
mod sea_orm_support;
#[cfg(feature = "sea-orm")]
pub use self::sea_orm_support::*;

/// This is a wrapper around a String. Which can only be created,
/// by validating the string is an email.
///
/// Once wrapped, you can get the original string back by using this as a reference,
/// using `into`, or calling `to_string`.
///
/// This supports `serde` using the `serde` feature (which is on by default).
/// Allowing you to serialise and deserialise as you want.
#[derive(Clone, Debug, PartialEq)]
pub struct Email {
    raw_email: String,
}

impl Email {
    /// Creates a new Email, that wraps the string given.
    ///
    /// If the given string doesn't look like a valid email,
    /// then this will return an EmailError.
    pub fn new(raw_email: String) -> Result<Self, EmailError> {
        if !is_valid_email(&raw_email) {
            let err = EmailError::Invalid { raw_email };
            return Err(err);
        }

        Ok(Self { raw_email })
    }

    /// Returns a new Email, where the email has been uppercased.
    ///
    /// This is useful if you want to compare two Email's in a case insensitive way.
    pub fn to_lowercase(&self) -> Self {
        Self {
            raw_email: self.raw_email.to_lowercase(),
        }
    }

    /// Returns a new Email, where the internal email has been uppercased.
    pub fn to_uppercase(&self) -> Self {
        Self {
            raw_email: self.raw_email.to_lowercase(),
        }
    }
}

impl Display for Email {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(f, "{}", self.raw_email)
    }
}

impl Email {
    pub fn as_str<'a>(&'a self) -> &'a str {
        &self.raw_email
    }

    pub fn as_string<'a>(&'a self) -> &'a String {
        &self.raw_email
    }

    pub fn to_string(self) -> String {
        self.raw_email
    }
}

impl Into<String> for Email {
    fn into(self) -> String {
        self.raw_email
    }
}

impl AsRef<str> for Email {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl AsRef<String> for Email {
    fn as_ref(&self) -> &String {
        self.as_string()
    }
}

#[cfg(test)]
mod test_new {
    use super::*;

    #[test]
    fn it_should_accept_a_valid_email() {
        let maybe_email = Email::new("john@example.com".to_string());

        assert!(maybe_email.is_ok());
    }

    #[test]
    fn it_should_not_accept_a_non_valid_email() {
        let maybe_email = Email::new("donkeys".to_string());

        assert!(maybe_email.is_err());
    }

    #[test]
    fn it_should_not_accept_a_domain_on_its_own() {
        let maybe_email = Email::new("@example.com".to_string());

        assert!(maybe_email.is_err());
    }

    #[test]
    fn it_should_not_accept_a_user_part_on_its_own() {
        let maybe_email = Email::new("john@".to_string());

        assert!(maybe_email.is_err());
    }

    #[test]
    fn it_should_not_accept_an_empty_string() {
        let maybe_email = Email::new("".to_string());

        assert!(maybe_email.is_err());
    }
}
