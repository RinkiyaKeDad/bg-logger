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
    model::PlayParticipantModel,
    schema::{PlayParticipantSchema, UpdatePlayParticipantSchema},
};

pub async fn create_play_participant_handler(
    State(data): State<Arc<AppState>>,
    Json(body): Json<PlayParticipantSchema>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let play_participant = sqlx::query_as!(
        PlayParticipantModel,
        r#"INSERT INTO play_participants (play_id, player_id) VALUES ($1, $2) RETURNING *"#,
        &body.play_id,
        &body.player_id,
    )
    .fetch_one(&data.db)
    .await
    .map_err(|e| e.to_string());

    if let Err(err) = play_participant {
        return Err((
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({"status": "error","message": format!("{:?}", err)})),
        ));
    }

    let play_participant_response = json!({
            "status": "success",
            "data": json!({
                "play_participant": play_participant
        })
    });

    Ok(Json(play_participant_response))
}

pub async fn play_participant_list_handler(
    State(data): State<Arc<AppState>>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let play_participants =
        sqlx::query_as!(PlayParticipantModel, r#"SELECT * FROM play_participants"#)
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
        "count": play_participants.len(),
        "play_participants": play_participants
    });

    Ok(Json(json_response))
}

pub async fn get_play_participants_handler(
    Path(play_id): Path<Uuid>,
    State(data): State<Arc<AppState>>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let play_participants = sqlx::query_as!(
        PlayParticipantModel,
        r#"SELECT * FROM play_participants WHERE play_id = $1"#,
        &play_id
    )
    .fetch_all(&data.db)
    .await
    .map_err(|e| {
        let error_response = serde_json::json!({
            "status": "error",
            "message": format!("Database error: {}", e),
        });
        (StatusCode::INTERNAL_SERVER_ERROR, Json(error_response))
    })?;

    let play_participants_response = serde_json::json!({
        "status": "success",
        "count": play_participants.len(),
        "data": json!({
            "play_participants": play_participants
        })
    });

    Ok(Json(play_participants_response))
}

pub async fn delete_play_participant_handler(
    Path((play_id, player_id)): Path<(Uuid, Uuid)>,
    State(data): State<Arc<AppState>>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let deleted_participant = sqlx::query_as!(
        PlayParticipantModel,
        r#"DELETE FROM play_participants WHERE play_id = $1 AND player_id = $2 RETURNING *"#,
        &play_id,
        &player_id
    )
    .fetch_one(&data.db)
    .await
    .map_err(|e| match e {
        sqlx::Error::RowNotFound => (
            StatusCode::NOT_FOUND,
            Json(json!({
                "status": "error",
                "message": "Play participant not found"
            })),
        ),
        _ => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({
                "status": "error",
                "message": format!("Database error: {}", e)
            })),
        ),
    })?;

    let response = json!({
        "status": "success",
        "message": "Play participant deleted successfully",
        "data": {
            "deleted_participant": deleted_participant
        }
    });

    Ok(Json(response))
}

pub async fn update_play_participant_handler(
    Path((play_id, player_id)): Path<(Uuid, Uuid)>,
    State(data): State<Arc<AppState>>,
    Json(body): Json<UpdatePlayParticipantSchema>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let query_result = sqlx::query_as!(
        PlayParticipantModel,
        r#"SELECT * FROM play_participants WHERE play_id = $1 AND player_id = $2"#,
        &play_id,
        &player_id
    )
    .fetch_one(&data.db)
    .await;

    let play_participant: PlayParticipantModel = match query_result {
        Ok(play_participant) => play_participant,
        Err(sqlx::Error::RowNotFound) => {
            let error_response = serde_json::json!({
                "status": "error",
                "message": format!("Play participant with ID: ({},{}) not found", play_id, player_id)
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

    let new_play_id = body.play_id.as_ref().unwrap_or(&play_participant.play_id);
    let new_player_id = body
        .player_id
        .as_ref()
        .unwrap_or(&play_participant.player_id);

    let updated_play_participant = sqlx::query_as!(
        PlayParticipantModel,
        r#"UPDATE play_participants SET play_id = $1, player_id = $2 WHERE play_id = $3 AND player_id = $4 RETURNING *"#,
        &new_play_id,
        &new_player_id,
        &play_id,
        &player_id
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
            "play_participant": updated_play_participant
        })
    });
    Ok(Json(response))
}
