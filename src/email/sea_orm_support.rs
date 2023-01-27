use ::sea_orm::entity::ActiveValue;
use ::sea_orm::entity::IntoActiveValue;
use ::sea_orm::error::DbErr;
use ::sea_orm::sea_query::table::ColumnType;
use ::sea_orm::sea_query::value::ArrayType;
use ::sea_orm::sea_query::value::Nullable;
use ::sea_orm::sea_query::value::ValueType;
use ::sea_orm::sea_query::value::ValueTypeErr;
use ::sea_orm::QueryResult;
use ::sea_orm::TryGetError;
use ::sea_orm::TryGetable;
use ::sea_orm::Value;
use ::std::convert::From;

use crate::Email;

impl From<Email> for Value {
    fn from(email: Email) -> Value {
        Value::String(Some(Box::new(email.to_string())))
    }
}

impl Nullable for Email {
    fn null() -> Value {
        Value::String(None)
    }
}

impl TryGetable for Email {
    fn try_get(res: &QueryResult, pre: &str, col: &str) -> Result<Self, TryGetError> {
        res.try_get::<Option<String>>(pre, col)
            .map_err(|db_err| TryGetError::DbErr(db_err))
            .and_then(|maybe_raw| match maybe_raw {
                Some(raw) => Email::new(raw).map_err(|err| {
                    let db_err = DbErr::Custom(err.to_string());
                    TryGetError::DbErr(db_err)
                }),
                None => Err(TryGetError::Null(col.to_string())),
            })
    }
}

impl ValueType for Email {
    fn try_from(value: Value) -> Result<Self, ValueTypeErr> {
        match value {
            Value::String(Some(raw_email)) => Email::new(*raw_email).map_err(|_| ValueTypeErr),
            _ => Err(ValueTypeErr),
        }
    }

    fn type_name() -> String {
        "Email".to_string()
    }

    fn array_type() -> ArrayType {
        ArrayType::String
    }

    fn column_type() -> ColumnType {
        ColumnType::Text
    }
}

impl IntoActiveValue<Email> for Email {
    fn into_active_value(self) -> ActiveValue<Self> {
        ActiveValue::Set(self)
    }
}

#[cfg(test)]
mod test_try_getable {
    use super::*;
    use ::sea_orm::entity::prelude::*;

    #[test]
    fn it_should_compile_with_email() {
        #[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
        #[sea_orm(table_name = "test")]
        pub struct Model {
            #[sea_orm(primary_key)]
            pub id: i32,
            pub email: Email,
        }

        #[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
        pub enum Relation {}

        impl ActiveModelBehavior for ActiveModel {}

        let email = Email::new("joe@example.com".to_string()).unwrap();
        let model = Model {
            id: 123,
            // Please don't share my private email address.
            email: email.clone(),
        };

        // If it reaches this point, it means the above compiled fine.
        assert_eq!(model.email, email);
    }

    #[test]
    fn it_should_compile_with_optional_email_as_none() {
        #[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
        #[sea_orm(table_name = "test")]
        pub struct Model {
            #[sea_orm(primary_key)]
            pub id: i32,
            pub email: Option<Email>,
        }

        #[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
        pub enum Relation {}

        impl ActiveModelBehavior for ActiveModel {}

        let email = None;
        let model = Model {
            id: 123,
            email: email.clone(),
        };

        // If it reaches this point, it means the above compiled fine.
        assert_eq!(model.email, email);
    }

    #[test]
    fn it_should_compile_with_optional_some_email() {
        #[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
        #[sea_orm(table_name = "test")]
        pub struct Model {
            #[sea_orm(primary_key)]
            pub id: i32,
            pub email: Option<Email>,
        }

        #[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
        pub enum Relation {}

        impl ActiveModelBehavior for ActiveModel {}

        let email = Some(Email::new("joe@example.com".to_string()).unwrap());
        let model = Model {
            id: 123,
            email: email.clone(),
        };

        // If it reaches this point, it means the above compiled fine.
        assert_eq!(model.email, email);
    }
}
