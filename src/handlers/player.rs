use std::sync::Arc;

use axum::{
    Json,
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
};
use uuid::Uuid;

use serde_json::json;

use crate::{AppState, model::PlayerModel, schema::PlayerSchema};

pub async fn create_player_handler(
    State(data): State<Arc<AppState>>,
    Json(body): Json<PlayerSchema>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let id = uuid::Uuid::new_v4();
    let player = sqlx::query_as!(
        PlayerModel,
        r#"INSERT INTO players (id, name, is_owner) VALUES ($1, $2, $3) RETURNING *"#,
        &id,
        &body.name,
        &body.is_owner.unwrap_or(false),
    )
    .fetch_one(&data.db)
    .await
    .map_err(|e| e.to_string());

    if let Err(err) = player {
        if err.contains("duplicate key value") {
            if err.contains("uniq_single_owner") {
                let error_response = serde_json::json!({
                    "status": "error",
                    "message": "Only one owner is allowed",
                });
                return Err((StatusCode::CONFLICT, Json(error_response)));
            } else {
                let error_response = serde_json::json!({
                    "status": "error",
                    "message": "Player name already exists",
                });
                return Err((StatusCode::CONFLICT, Json(error_response)));
            }
        }

        return Err((
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({"status": "error","message": format!("{:?}", err)})),
        ));
    }

    let player_response = json!({
            "status": "success",
            "data": json!({
                "player": player
        })
    });

    Ok(Json(player_response))
}

pub async fn player_list_handler(
    State(data): State<Arc<AppState>>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let players = sqlx::query_as!(PlayerModel, r#"SELECT * FROM players ORDER BY name"#)
        .fetch_all(&data.db)
        .await
        .map_err(|e| {
            let error_response = serde_json::json!({
                "status": "error",
                "message": format!("Database error: {}", e),
            });
            (StatusCode::INTERNAL_SERVER_ERROR, Json(error_response))
        })?;

    let json_response = serde_json::json!({
        "status": "ok",
        "count": players.len(),
        "players": players
    });

    Ok(Json(json_response))
}

pub async fn get_player_handler(
    Path(player_id): Path<Uuid>,
    State(data): State<Arc<AppState>>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let query_result = sqlx::query_as!(
        PlayerModel,
        r#"SELECT * FROM players WHERE id = $1"#,
        &player_id
    )
    .fetch_one(&data.db)
    .await;

    match query_result {
        Ok(player) => {
            let player_response = serde_json::json!({
                "status" : "success",
                "data": serde_json::json!({
                    "player": player
                })
            });

            Ok(Json(player_response))
        }
        Err(sqlx::Error::RowNotFound) => {
            let error_response = serde_json::json!({
                "status": "fail",
                "message": format!("Player with ID: {} not found", player_id)
            });
            Err((StatusCode::NOT_FOUND, Json(error_response)))
        }
        Err(e) => Err((
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({"status": "error", "message": format!("{:?}", e)})),
        )),
    }
}

pub async fn delete_player_handler(
    Path(player_id): Path<Uuid>,
    State(data): State<Arc<AppState>>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let query_result = sqlx::query_as!(
        PlayerModel,
        r#"DELETE FROM players WHERE id = $1 RETURNING *"#,
        &player_id
    )
    .fetch_one(&data.db)
    .await
    .map_err(|e| match e {
        sqlx::Error::RowNotFound => (
            StatusCode::NOT_FOUND,
            Json(json!({
                "status": "error",
                "message": "Player not found"
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
        "message": "Player deleted successfully",
        "data": {
            "deleted_player" : query_result
        }
    });

    Ok(Json(response))
}

pub async fn update_player_handler(
    Path(id): Path<Uuid>,
    State(data): State<Arc<AppState>>,
    Json(body): Json<PlayerSchema>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let query_result = sqlx::query_as!(PlayerModel, r#"SELECT * FROM players WHERE id = $1"#, &id)
        .fetch_one(&data.db)
        .await;

    let player = match query_result {
        Ok(player) => player,
        Err(sqlx::Error::RowNotFound) => {
            let error_response = serde_json::json!({
                "status": "error",
                "message": format!("Player with ID: {} not found", id)
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

    let new_name = &body.name;

    let updated_player = sqlx::query_as!(
        PlayerModel,
        r#"UPDATE players SET name = $1 WHERE id = $2 RETURNING *"#,
        &new_name,
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
            "player": updated_player
        })
    });
    Ok(Json(response))
}
