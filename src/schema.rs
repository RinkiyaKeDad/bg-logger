use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Schema for creating or updating a player
#[derive(Serialize, Deserialize, Debug)]
pub struct GameSchema {
    pub name: String,
    pub creator_name: String,
}

/// Schema for updating an existing note
#[derive(Serialize, Deserialize, Debug)]
pub struct UpdateGameSchema {
    pub name: Option<String>,
    pub creator_name: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PlayerSchema {
    pub name: String,
    pub is_owner: Option<bool>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PlaySchema {
    pub game_id: Uuid,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PlayParticipantSchema {
    pub play_id: Uuid,
    pub player_id: Uuid,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct UpdatePlayParticipantSchema {
    pub play_id: Option<Uuid>,
    pub player_id: Option<Uuid>,
}
