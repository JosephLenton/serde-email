use ::std::convert::AsRef;
use ::std::convert::Into;
use ::std::fmt::Debug;
use ::std::fmt::Display;
use ::std::fmt::Formatter;
use ::std::fmt::Result as FmtResult;
use ::serde::Deserialize;
use ::serde::Deserializer;
use ::serde::Serialize;
use ::serde::Serializer;

use crate::is_valid_email;
use crate::EmailError;
use crate::EmailVisitor;

#[derive(Clone, Debug, PartialEq)]
pub struct Email {
    raw_email: String,
}

impl Email {
    pub fn new(raw_email: String) -> Result<Self, EmailError> {
        if !is_valid_email(&raw_email) {
            let err = EmailError::Invalid { raw_email };
            return Err(err);
        }

        Ok(Self { raw_email })
    }
}

impl Serialize for Email {
  fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
  where
      S: Serializer,
  {
      serializer.serialize_str(&self.raw_email)
  }
}

impl<'de> Deserialize<'de> for Email {
  fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
  where
      D: Deserializer<'de>,
  {
      deserializer.deserialize_string(EmailVisitor)
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
