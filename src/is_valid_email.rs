use ::email_address::EmailAddress;
use ::std::convert::AsRef;

pub fn is_valid_email<S>(raw: S) -> bool
where
    S: AsRef<str>,
{
    EmailAddress::is_valid(raw.as_ref())
}
