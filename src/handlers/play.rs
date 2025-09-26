use std::sync::Arc;

use axum::{
    Json,
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
};
use uuid::Uuid;

use serde_json::json;

use crate::{AppState, model::PlayModel, schema::PlaySchema};

pub async fn create_play_handler(
    State(data): State<Arc<AppState>>,
    Json(body): Json<PlaySchema>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let play = sqlx::query_as!(
        PlayModel,
        r#"INSERT INTO plays (game_id) VALUES ($1) RETURNING *"#,
        &body.game_id,
    )
    .fetch_one(&data.db)
    .await
    .map_err(|e| e.to_string());

    if let Err(err) = play {
        return Err((
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({"status": "error","message": format!("{:?}", err)})),
        ));
    }

    let play_response = json!({
            "status": "success",
            "data": json!({
                "play": play
        })
    });

    Ok(Json(play_response))
}

pub async fn play_list_handler(
    State(data): State<Arc<AppState>>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let plays = sqlx::query_as!(PlayModel, r#"SELECT * FROM plays ORDER BY game_id"#)
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
        "count": plays.len(),
        "plays": plays
    });

    Ok(Json(json_response))
}

pub async fn get_play_handler(
    Path(play_id): Path<Uuid>,
    State(data): State<Arc<AppState>>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let query_result = sqlx::query_as!(PlayModel, r#"SELECT * FROM plays WHERE id = $1"#, &play_id)
        .fetch_one(&data.db)
        .await;

    match query_result {
        Ok(play) => {
            let play_response = serde_json::json!({
                "status" : "success",
                "data": json!({
                    "play": play
                })
            });

            Ok(Json(play_response))
        }
        Err(sqlx::Error::RowNotFound) => {
            let error_response = json!({
                "status": "fail",
                "message": format!("Play with ID: {} not found", play_id)
            });
            Err((StatusCode::NOT_FOUND, Json(error_response)))
        }
        Err(e) => Err((
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({"status": "error", "message": format!("{:?}", e)})),
        )),
    }
}

pub async fn delete_play_handler(
    Path(play_id): Path<Uuid>,
    State(data): State<Arc<AppState>>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let query_result = sqlx::query_as!(
        PlayModel,
        r#"DELETE FROM plays WHERE id = $1 RETURNING *"#,
        &play_id
    )
    .fetch_one(&data.db)
    .await
    .map_err(|e| match e {
        sqlx::Error::RowNotFound => (
            StatusCode::NOT_FOUND,
            Json(json!({
                "status": "error",
                "message": "Play not found"
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
        "message": "Play deleted successfully",
        "data": {
            "deleted_play" : query_result
        }
    });

    Ok(Json(response))
}

pub async fn update_play_handler(
    Path(id): Path<Uuid>,
    State(data): State<Arc<AppState>>,
    Json(body): Json<PlaySchema>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let query_result = sqlx::query_as!(PlayModel, r#"SELECT * FROM plays WHERE id = $1"#, &id)
        .fetch_one(&data.db)
        .await;

    let _play = match query_result {
        Ok(play) => play,
        Err(sqlx::Error::RowNotFound) => {
            let error_response = serde_json::json!({
                "status": "error",
                "message": format!("Play with ID: {} not found", id)
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

    let new_game_id = &body.game_id;

    let updated_play = sqlx::query_as!(
        PlayModel,
        r#"UPDATE plays SET game_id = $1 WHERE id = $2 RETURNING *"#,
        &new_game_id,
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
            "play": updated_play
        })
    });
    Ok(Json(response))
}
