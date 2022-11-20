use ::std::fmt::Formatter;
use ::std::fmt::Result as FmtResult;
use ::serde::de::Visitor;
use ::serde::de::Error as SerdeDeError;

use crate::Email;

pub struct EmailVisitor;

impl<'de> Visitor<'de> for EmailVisitor {
    type Value = Email;

    fn expecting(&self, formatter: &mut Formatter) -> FmtResult {
        formatter.write_str("a valid email address")
    }

    fn visit_string<E>(self, raw_email: String) -> Result<Self::Value, E>
        where
            E: SerdeDeError
    {
        Email::new(raw_email).map_err(|err| {
          let msg = format!("{}", err);
          SerdeDeError::custom(msg)
        })
    }
}
