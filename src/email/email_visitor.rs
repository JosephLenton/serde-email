use ::serde::de::Error as SerdeDeError;
use ::serde::de::Visitor;
use ::std::fmt::Formatter;
use ::std::fmt::Result as FmtResult;

use crate::Email;

pub struct EmailVisitor;

impl<'de> Visitor<'de> for EmailVisitor {
    type Value = Email;

    fn expecting(&self, formatter: &mut Formatter) -> FmtResult {
        formatter.write_str("a valid email address")
    }

    fn visit_str<E>(self, raw_email: &str) -> Result<Self::Value, E>
    where
        E: SerdeDeError,
    {
        return Email::from_str(raw_email).map_err(|err| {
            let msg = format!("{}", err);
            SerdeDeError::custom(msg)
        });
    }

    fn visit_string<E>(self, raw_email: String) -> Result<Self::Value, E>
    where
        E: SerdeDeError,
    {
        return Email::from_string(raw_email).map_err(|err| {
            let msg = format!("{}", err);
            SerdeDeError::custom(msg)
        });
    }
}
