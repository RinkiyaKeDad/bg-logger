use std::sync::Arc;

use axum::{
    Router,
    routing::{get, patch, post},
};

use crate::{
    AppState,
    handlers::{
        create_game_handler, create_player_handler, delete_game_handler, delete_player_handler,
        game_list_handler, get_game_handler, get_player_handler, player_list_handler,
        update_game_handler, update_player_handler,
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
        .with_state(app_state)
}
