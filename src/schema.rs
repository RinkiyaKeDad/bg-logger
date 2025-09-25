use serde::{Deserialize, Serialize};

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
