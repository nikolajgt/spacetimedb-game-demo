use sqlx::FromRow;
use uuid::Uuid;

#[derive(FromRow)]
pub struct Character {
    pub id: Uuid,
    pub user_id: Uuid,
    pub name: String,
    pub level: i16,
    pub created_at: chrono::DateTime<chrono::Utc>,
}
