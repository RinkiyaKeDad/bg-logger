use std::sync::Arc;

use axum::{
    Json,
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
};
use uuid::Uuid;

use serde_json::json;

use crate::{
    AppState,
    model::GameModel,
    schema::{GameSchema, UpdateGameSchema},
};

pub async fn create_game_handler(
    State(data): State<Arc<AppState>>,
    Json(body): Json<GameSchema>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let id = uuid::Uuid::new_v4();
    let game = sqlx::query_as!(
        GameModel,
        r#"INSERT INTO games (id, name, creator_name) VALUES ($1, $2, $3) RETURNING *"#,
        &id,
        &body.name,
        &body.creator_name,
    )
    .fetch_one(&data.db)
    .await
    .map_err(|e| e.to_string());

    if let Err(err) = game {
        if err.contains("duplicate key value") {
            let error_response = serde_json::json!({
                "status": "error",
                "message": "Game already exists",
            });
            return Err((StatusCode::CONFLICT, Json(error_response)));
        }

        return Err((
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({"status": "error","message": format!("{:?}", err)})),
        ));
    }

    let game_response = json!({
            "status": "success",
            "data": json!({
                "game": game
        })
    });

    Ok(Json(game_response))
}

pub async fn game_list_handler(
    State(data): State<Arc<AppState>>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    // Query with macro
    let games = sqlx::query_as!(GameModel, r#"SELECT * FROM games ORDER by name"#)
        .fetch_all(&data.db)
        .await
        .map_err(|e| {
            let error_response = serde_json::json!({
                "status": "error",
                "message": format!("Database error: { }", e),
            });
            (StatusCode::INTERNAL_SERVER_ERROR, Json(error_response))
        })?;

    let json_response = serde_json::json!({
        "status": "ok",
        "count": games.len(),
        "notes": games
    });

    Ok(Json(json_response))
}

pub async fn get_game_handler(
    Path(game_id): Path<Uuid>,
    State(data): State<Arc<AppState>>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let query_result = sqlx::query_as!(GameModel, r#"SELECT * FROM games WHERE id = $1"#, &game_id)
        .fetch_one(&data.db)
        .await;

    match query_result {
        Ok(game) => {
            let game_response = serde_json::json!({
                "status" : "success",
                "data": serde_json::json!({
                    "game": game
                })
            });

            Ok(Json(game_response))
        }
        Err(sqlx::Error::RowNotFound) => {
            let error_response = serde_json::json!({
                "status": "fail",
                "message": format!("Game with ID: {} not found", game_id)
            });
            Err((StatusCode::NOT_FOUND, Json(error_response)))
        }
        Err(e) => Err((
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({"status": "error", "message": format!("{:?}", e)})),
        )),
    }
}

pub async fn delete_game_handler(
    Path(game_id): Path<Uuid>,
    State(data): State<Arc<AppState>>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let query_result = sqlx::query_as!(
        GameModel,
        r#"DELETE FROM games WHERE id = $1 RETURNING *"#,
        &game_id
    )
    .fetch_one(&data.db)
    .await
    .map_err(|e| match e {
        sqlx::Error::RowNotFound => (
            StatusCode::NOT_FOUND,
            Json(json!({
                "status": "error",
                "message": "Game not found"
            })),
        ),
        _ => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({
                "status": "error",
                "message": format!("{:?}", e)
            })),
        ),
    })?;

    let response = json!({
        "status": "success",
        "message": "Game delete successfully",
        "data": {
            "deleted_game" : query_result
        }
    });

    Ok(Json(response))
}

pub async fn update_game_handler(
    Path(id): Path<Uuid>,
    State(data): State<Arc<AppState>>,
    Json(body): Json<UpdateGameSchema>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let query_result = sqlx::query_as!(GameModel, r#"SELECT * FROM games WHERE id = $1"#, &id)
        .fetch_one(&data.db)
        .await;

    let game = match query_result {
        Ok(game) => game,
        Err(sqlx::Error::RowNotFound) => {
            let error_response = serde_json::json!({
                "status": "error",
                "message": format!("Game with ID: {} not found", id)
            });
            return Err((StatusCode::NOT_FOUND, Json(error_response)));
        }
        Err(e) => {
            return Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({
                    "status": "error",
                    "message": format!("{:?}",e)
                })),
            ));
        }
    };

    let new_name = body.name.as_ref().unwrap_or(&game.name);
    let new_creator = body.creator_name.as_ref().unwrap_or(&game.creator_name);

    let updated_game = sqlx::query_as!(
        GameModel,
        r#"UPDATE games SET name = $1, creator_name = $2 WHERE id = $3 RETURNING *"#,
        &new_name,
        &new_creator,
        &id
    )
    .fetch_one(&data.db)
    .await
    .map_err(|e| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({
                "status": "error",
                "message": format!("{:?}", e)
            })),
        )
    })?;

    let response = json!({
        "status": "success",
        "data": json!({
            "player": updated_game
        })
    });
    Ok(Json(response))
}
