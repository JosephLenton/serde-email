use ::email_address::EmailAddress;
use ::std::convert::AsRef;

/// Tests if the given string is a valid email or not.
///
/// The underlying implementation is by the [email_address crate](https://crates.io/crates/email_address).
pub fn is_valid_email<S>(raw: S) -> bool
where
    S: AsRef<str>,
{
    EmailAddress::is_valid(raw.as_ref())
}
