use ::sqlx::database::HasArguments;
use ::sqlx::database::HasValueRef;
use ::sqlx::decode::Decode;
use ::sqlx::encode::Encode;
use ::sqlx::encode::IsNull;
use ::sqlx::Database;

use crate::Email;

impl<'q, DB: Database> Encode<'q, DB> for Email
where
    String: Encode<'q, DB>,
{
    fn encode_by_ref(&self, buf: &mut <DB as HasArguments<'q>>::ArgumentBuffer) -> IsNull {
        <String as Encode<'q, DB>>::encode_by_ref(self.as_string(), buf)
    }
    fn produces(&self) -> Option<DB::TypeInfo> {
        <String as Encode<'q, DB>>::produces(self.as_string())
    }
    fn size_hint(&self) -> usize {
        <String as Encode<'q, DB>>::size_hint(self.as_string())
    }
}

impl<'r, DB: Database> Decode<'r, DB> for Email
where
    String: Decode<'r, DB>,
{
    fn decode(
        value: <DB as HasValueRef<'r>>::ValueRef,
    ) -> Result<Self, Box<dyn ::std::error::Error + 'static + Send + Sync>> {
        let string = <String as Decode<'r, DB>>::decode(value)?;
        let result = Email::new(string)?;

        Ok(result)
    }
}
