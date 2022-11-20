use ::serde::Deserialize;
use ::serde::Deserializer;
use ::serde::Serialize;
use ::serde::Serializer;

use crate::Email;
use crate::EmailVisitor;

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

#[cfg(test)]
mod test_serialisation {
    use super::*;
    use ::serde::Deserialize;
    use ::serde::Serialize;
    use ::serde_json;

    #[derive(Serialize, Deserialize)]
    struct Person {
        name: String,
        email: Email,
    }

    #[test]
    fn it_should_serialise_email_from_string() {
        let email = Email::new("john@example.com".to_string()).unwrap();
        let raw = serde_json::to_string(&email).unwrap();

        assert_eq!(raw, r#""john@example.com""#);
    }
}

#[cfg(test)]
mod test_deserialisation {
    use super::*;
    use ::serde::Deserialize;
    use ::serde::Serialize;
    use ::serde_json;

    #[derive(Serialize, Deserialize)]
    struct Person {
        name: String,
        email: Email,
    }

    #[test]
    fn it_should_not_deserialise_non_email_from_string() {
        let raw_email = "donkeys";

        let result = serde_json::from_str::<Email>(raw_email);

        assert!(result.is_err());
    }

    #[test]
    fn it_should_deserialise_not_structs_from_raw_with_non_email() {
        let data = r#"
        {
            "name": "John Doe",
            "email": "donkeys"
        }"#;

        let result = serde_json::from_str::<Person>(data);

        assert!(result.is_err());
    }

    #[test]
    fn it_should_deserialise_email_from_string() {
        let raw_json_email = r#""john@example.com""#;

        let email: Email = serde_json::from_str(raw_json_email).unwrap();

        assert_eq!(email, Email::new("john@example.com".to_string()).unwrap());
    }

    #[test]
    fn it_should_deserialise_structs_from_raw() {
        let data = r#"
        {
            "name": "John Doe",
            "email": "john@example.com"
        }"#;

        let person = serde_json::from_str::<Person>(data).unwrap();

        assert_eq!(
            person.email,
            Email::new("john@example.com".to_string()).unwrap()
        );
    }
}
