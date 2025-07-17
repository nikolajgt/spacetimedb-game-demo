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

#[derive(FromRow)]
pub struct CharacterMovement {
    pub id: Uuid,
    pub character_id: Uuid,

    pub pos_x: f32,
    pub pos_y: f32,
    pub pos_z: f32,

    pub dir_x: f32,
    pub dir_y: f32,
    pub dir_z: f32,

    pub mode: i16,
}
