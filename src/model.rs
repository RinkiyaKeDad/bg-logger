use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Database model for a game
#[derive(Debug, Deserialize, Serialize, sqlx::FromRow)]
pub struct GameModel {
    pub id: Uuid,
    pub name: String,
    pub creator_name: String,
    pub created_at: chrono::DateTime<chrono::Utc>,
}

/// Database model for a player
#[derive(Debug, Deserialize, Serialize, sqlx::FromRow)]
pub struct PlayerModel {
    pub id: Uuid,
    pub name: String,
    pub is_owner: bool,
    pub created_at: chrono::DateTime<chrono::Utc>,
}

/// Database model for a play
#[derive(Debug, Deserialize, Serialize, sqlx::FromRow)]
pub struct PlayModel {
    pub id: Uuid,
    pub game_id: Uuid,
    pub created_at: chrono::DateTime<chrono::Utc>,
}

/// Database model for a play participant
#[derive(Debug, Deserialize, Serialize, sqlx::FromRow)]
pub struct PlayParticipantModel {
    pub play_id: Uuid,
    pub player_id: Uuid,
}
