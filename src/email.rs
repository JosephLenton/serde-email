use ::std::convert::AsRef;
use ::std::convert::From;
use ::std::fmt::Debug;
use ::std::fmt::Display;
use ::std::fmt::Formatter;
use ::std::fmt::Result as FmtResult;
use ::std::str::FromStr;

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

/// A validated Email object.
///
/// These can be created from a string using `Email::from_string`,
/// `Email::from_str`, or `String::parse`.
///
/// Once built you can turn this like a `String`, use with Serde, or Sea Orm.
///
/// Note that Email objects _are_ case sensetive.
/// The email addresses `Email::from_str("bob@example.com")` and `Email::from_str("BoB@example.com")`,
/// will not be equal to each other.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct Email {
    raw_email: String,
}

impl Email {
    /// Creates a new Email, from the `String` given.
    ///
    /// If the given string doesn't look like a valid email,
    /// then this will return an EmailError.
    pub fn from_string(raw_email: String) -> Result<Self, EmailError> {
        if !is_valid_email(&raw_email) {
            let err = EmailError::Invalid { raw_email };
            return Err(err);
        }

        Ok(Self { raw_email })
    }

    /// Creates a new Email, from the `str` given.
    ///
    /// If the given string doesn't look like a valid email,
    /// then this will return an EmailError.
    pub fn from_str<S>(raw_email: S) -> Result<Self, EmailError>
    where
        S: AsRef<str>,
    {
        Self::from_string(raw_email.as_ref().to_string())
    }

    /// Returns a new Email, where the email has been uppercased.
    pub fn to_lowercase(&self) -> Self {
        Self {
            raw_email: self.raw_email.to_lowercase(),
        }
    }

    /// Returns a new Email, where the internal email has been uppercased.
    pub fn to_uppercase(&self) -> Self {
        Self {
            raw_email: self.raw_email.to_uppercase(),
        }
    }

    pub fn as_str<'a>(&'a self) -> &'a str {
        &self.raw_email
    }
}

/// This is a common default, provided in use for stuff like tests.
///
/// The default email is `default@example.com`.
impl Default for Email {
    fn default() -> Self {
        Self::from_str("default@example.com").expect("Default Email should always be valid")
    }
}

impl Display for Email {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(f, "{}", self.raw_email)
    }
}

impl From<Email> for String {
    fn from(email: Email) -> Self {
        email.raw_email
    }
}

impl AsRef<str> for Email {
    fn as_ref(&self) -> &str {
        &self.raw_email
    }
}

impl AsRef<String> for Email {
    fn as_ref(&self) -> &String {
        &self.raw_email
    }
}

impl FromStr for Email {
    type Err = EmailError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Email::from_str(s)
    }
}

impl TryFrom<String> for Email {
    type Error = EmailError;

    fn try_from(raw: String) -> Result<Self, Self::Error> {
        Email::from_string(raw)
    }
}

impl<'a> TryFrom<&'a str> for Email {
    type Error = EmailError;

    fn try_from(raw: &'a str) -> Result<Self, Self::Error> {
        Email::from_str(raw)
    }
}

impl<'a> PartialEq<&'a str> for Email {
    fn eq(&self, other: &&'a str) -> bool {
        self.raw_email == *other
    }
}

impl PartialEq<String> for Email {
    fn eq(&self, other: &String) -> bool {
        self.raw_email == *other
    }
}

#[cfg(test)]
mod test_from_string {
    use super::*;

    #[test]
    fn it_should_accept_a_valid_email() {
        let maybe_email = Email::from_string("john@example.com".to_string());

        assert!(maybe_email.is_ok());
    }

    #[test]
    fn it_should_not_accept_a_non_valid_email() {
        let maybe_email = Email::from_string("foxes".to_string());

        assert!(maybe_email.is_err());
    }

    #[test]
    fn it_should_not_accept_a_domain_on_its_own() {
        let maybe_email = Email::from_string("@example.com".to_string());

        assert!(maybe_email.is_err());
    }

    #[test]
    fn it_should_not_accept_a_user_part_on_its_own() {
        let maybe_email = Email::from_string("john@".to_string());

        assert!(maybe_email.is_err());
    }

    #[test]
    fn it_should_not_accept_an_empty_string() {
        let maybe_email = Email::from_string("".to_string());

        assert!(maybe_email.is_err());
    }
}

#[cfg(test)]
mod test_from_str {
    use super::*;

    #[test]
    fn it_should_accept_a_valid_email() {
        let maybe_email = Email::from_str("john@example.com");

        assert!(maybe_email.is_ok());
    }

    #[test]
    fn it_should_not_accept_a_non_valid_email() {
        let maybe_email = Email::from_str("foxes");

        assert!(maybe_email.is_err());
    }
}

#[cfg(test)]
mod test_try_from {
    use super::*;

    #[test]
    fn it_should_parse_valid_email_from_str() {
        let email: Email = "fox@example.com".try_into().unwrap();

        assert_eq!(email, "fox@example.com");
    }

    #[test]
    fn it_should_not_parse_invalid_email_from_str() {
        let maybe_email: Result<Email, EmailError> = "🦊🦊🦊".try_into();

        assert!(maybe_email.is_err());
    }

    #[test]
    fn it_should_parse_valid_email_from_string() {
        let email: Email = "fox@example.com".to_string().try_into().unwrap();

        assert_eq!(email, "fox@example.com");
    }

    #[test]
    fn it_should_not_parse_invalid_email_from_string() {
        let maybe_email: Result<Email, EmailError> = "🦊🦊🦊".to_string().try_into();

        assert!(maybe_email.is_err());
    }
}

#[cfg(test)]
mod test_parse {
    use super::*;

    #[test]
    fn it_should_parse_valid_email_from_string() {
        let email: Email = "fox@example.com".parse().unwrap();

        assert_eq!(email, "fox@example.com");
    }

    #[test]
    fn it_should_not_parse_invalid_email_from_string() {
        let maybe_email: Result<Email, EmailError> = "🦊🦊🦊".parse();

        assert!(maybe_email.is_err());
    }
}

#[cfg(test)]
mod test_display {
    use super::*;

    #[test]
    fn it_should_write_same_email_as_given() {
        let email: Email = "fox@example.com".parse().unwrap();
        let output: String = format!("{}", email);

        assert!(email == output);
        assert_eq!(output, "fox@example.com");
    }
}

#[cfg(test)]
mod test_default {
    use super::*;

    #[test]
    fn it_should_create_a_valid_default() {
        let email = Email::default();

        assert!(is_valid_email(&email));
    }
}

#[cfg(test)]
mod test_to_lowercase {
    use super::*;

    #[test]
    fn it_should_make_it_lowercase() {
        let email: Email = "JoE@eXaMpLe.com".parse().unwrap();

        assert_eq!(
            email.to_lowercase(),
            Email::from_str("joe@example.com").unwrap()
        );
    }

    #[test]
    fn it_should_not_change_already_lowercase() {
        let email: Email = "joe@example.com".parse().unwrap();

        assert_eq!(
            email.to_lowercase(),
            Email::from_str("joe@example.com").unwrap()
        );
    }
}

#[cfg(test)]
mod test_to_uppercase {
    use super::*;

    #[test]
    fn it_should_make_it_uppercase() {
        let email: Email = "JoE@eXaMpLe.com".parse().unwrap();

        assert_eq!(
            email.to_uppercase(),
            Email::from_str("JOE@EXAMPLE.COM").unwrap()
        );
    }

    #[test]
    fn it_should_not_change_already_uppercase() {
        let email: Email = "joe@example.com".parse().unwrap();

        assert_eq!(
            email.to_uppercase(),
            Email::from_str("JOE@EXAMPLE.COM").unwrap()
        );
    }
}

#[cfg(test)]
mod test_partial_eq {
    use super::*;

    #[test]
    fn it_should_be_equal_to_strs() {
        let email: Email = "joe@example.com".parse().unwrap();
        let is_equal = email == "joe@example.com";

        assert!(is_equal);
    }

    #[test]
    fn it_should_not_be_equal_to_different_strs() {
        let email: Email = "joe@example.com".parse().unwrap();
        let is_not_equal = email != "🦊@example.com";

        assert!(is_not_equal);
    }
}
