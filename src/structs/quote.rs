use sqlx::types::time::OffsetDateTime;

#[derive(Debug)]
pub struct Quote {
    pub quote_id: i32,
    pub user_id: i64,
    pub username: String,
    pub quote: String,
    pub added_by: i64,
    pub added_at: OffsetDateTime,
}
