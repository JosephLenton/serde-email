use ::sea_orm::entity::ActiveValue;
use ::sea_orm::entity::IntoActiveValue;
use ::sea_orm::error::DbErr;
use ::sea_orm::sea_query::table::ColumnType;
use ::sea_orm::sea_query::value::ArrayType;
use ::sea_orm::sea_query::value::ValueType;
use ::sea_orm::sea_query::value::ValueTypeErr;
use ::sea_orm::QueryResult;
use ::sea_orm::TryGetError;
use ::sea_orm::TryGetable;
use ::sea_orm::Value;
use ::std::convert::Into;

use crate::Email;

impl Into<Value> for Email {
    fn into(self) -> Value {
        Value::String(Some(Box::new(self.raw_email)))
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

// impl IntoValueTuple for Email {
//     fn into_value_tuple(self) -> ValueTuple {
//         ValueTuple::One(self.into())
//     }
// }

impl IntoActiveValue<Email> for Email {
    fn into_active_value(self) -> ActiveValue<Self> {
        ActiveValue::Set(self)
    }
}

// impl TryGetable for Email {
//     fn try_get(res: &QueryResult, pre: &str, col: &str) -> Result<Self, TryGetError> {
//         let column = format!("{}{}", pre, col);
//         match &res.row {
//             #[cfg(feature = "sqlx-mysql")]
//             QueryResultRow::SqlxMySql(row) => {
//                 use sqlx::Row;
//                 row.try_get::<Option<Email>, _>(column.as_str())
//                     .map_err(|e| TryGetError::DbErr(crate::sqlx_error_to_query_err(e)))
//                     .and_then(|opt| opt.ok_or(TryGetError::Null(column)))
//             }
//             #[cfg(feature = "sqlx-postgres")]
//             QueryResultRow::SqlxPostgres(row) => {
//                 use sqlx::Row;
//                 row.try_get::<Option<Email>, _>(column.as_str())
//                     .map_err(|e| TryGetError::DbErr(crate::sqlx_error_to_query_err(e)))
//                     .and_then(|opt| opt.ok_or(TryGetError::Null(column)))
//             }
//             #[cfg(feature = "sqlx-sqlite")]
//             QueryResultRow::SqlxSqlite(row) => {
//                 use sqlx::Row;
//                 row.try_get::<Option<Email>, _>(column.as_str())
//                     .map_err(|e| TryGetError::DbErr(crate::sqlx_error_to_query_err(e)))
//                     .and_then(|opt| opt.ok_or(TryGetError::Null(column)))
//             }
//             #[cfg(feature = "mock")]
//             #[allow(unused_variables)]
//             QueryResultRow::Mock(row) => row.try_get(column.as_str()).map_err(|e| {
//                 debug_print!("{:#?}", e.to_string());
//                 TryGetError::Null(column)
//             }),
//             #[allow(unreachable_patterns)]
//             _ => unreachable!(),
//         }
//     }
// }
