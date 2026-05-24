use sea_orm::{DbErr, RuntimeErr};

pub fn is_unique_violation(err: &DbErr) -> bool {
    if let DbErr::Query(RuntimeErr::SqlxError(sqlx_err)) = err
        && let Some(db_err) = sqlx_err.as_database_error()
    {
        return db_err.code() == Some(std::borrow::Cow::Borrowed("23505"));
    }
    false
}
