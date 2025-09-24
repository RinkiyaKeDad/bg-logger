use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Database model for a game
#[derive(Debug, Deserialize, Serialize, sqlx::FromRow)]
pub struct GameModel {
    pub id: Uuid,
    pub name: String,
    pub creator_name: String,
    pub created_at: Option<chrono::DateTime<chrono::Utc>>,
}
