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
use ::std::convert::Into;

use crate::Email;

impl From<Email> for Value {
    fn from(email: Email) -> Value {
        Value::String(Some(Box::new(self.to_string())))
    }
}

impl Nullable for Email {
    fn null() -> Value {
        Value::String(None)
    }
}

impl TryGetable for Email {
    fn try_get(res: &QueryResult, pre: &str, col: &str) -> Result<Self, TryGetError> {
        res.try_get::<String>(pre, col)
            .and_then(|raw_email| {
                Email::new(raw_email).map_err(|email_err| DbErr::Custom(email_err.to_string()))
            })
            .map_err(|db_err| TryGetError::DbErr(db_err))
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
