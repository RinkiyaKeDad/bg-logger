use std::sync::Arc;

use axum::{
    Router,
    routing::{delete, get, patch, post},
};

use crate::{
    AppState,
    handlers::{
        create_game_handler, create_play_handler, create_play_participant_handler,
        create_player_handler, delete_game_handler, delete_play_handler,
        delete_play_participant_handler, delete_player_handler, game_list_handler,
        get_game_handler, get_play_handler, get_play_participants_handler, get_player_handler,
        play_list_handler, play_participant_list_handler, player_list_handler, update_game_handler,
        update_play_handler, update_play_participant_handler, update_player_handler,
    },
};

pub fn create_router(app_state: Arc<AppState>) -> Router {
    Router::new()
        // Game routes
        .route("/api/games", post(create_game_handler))
        .route("/api/games", get(game_list_handler))
        .route(
            "/api/games/{id}",
            get(get_game_handler)
                .delete(delete_game_handler)
                .patch(update_game_handler),
        )
        // Player routes
        .route("/api/players", post(create_player_handler))
        .route("/api/players", get(player_list_handler))
        .route(
            "/api/players/{id}",
            get(get_player_handler)
                .delete(delete_player_handler)
                .patch(update_player_handler),
        )
        // Play routes
        .route("/api/plays", post(create_play_handler))
        .route("/api/plays", get(play_list_handler))
        .route(
            "/api/plays/{id}",
            get(get_play_handler)
                .delete(delete_play_handler)
                .patch(update_play_handler),
        )
        // Play Participant routes
        .route(
            "/api/playparticipants",
            post(create_play_participant_handler),
        )
        .route("/api/playparticipants", get(play_participant_list_handler))
        .route(
            "/api/plays/{play_id}/participants",
            get(get_play_participants_handler),
        )
        .route(
            "/api/plays/{play_id}/participants/{player_id}",
            delete(delete_play_participant_handler).patch(update_play_participant_handler),
        )
        .with_state(app_state)
}
