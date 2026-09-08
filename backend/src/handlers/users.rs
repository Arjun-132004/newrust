use crate::models::{ApiResponse, UpdateUserRequest, User, UserResponse};
use crate::utils::jwt::JwtClaims;
use axum::{extract::State, http::StatusCode, Extension, Json};
use sqlx::PgPool;

pub async fn get_current_user(
    State(pool): State<PgPool>,
    Extension(claims): Extension<JwtClaims>,
) -> Result<Json<ApiResponse<UserResponse>>, StatusCode> {
    match User::find_by_id(&pool, claims.sub).await {
        Ok(Some(user)) => Ok(Json(ApiResponse::success(user.into()))),
        Ok(None) => Err(StatusCode::NOT_FOUND),
        Err(e) => {
            tracing::error!("Failed to get user: {}", e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

pub async fn update_current_user(
    State(pool): State<PgPool>,
    Extension(claims): Extension<JwtClaims>,
    Json(request): Json<UpdateUserRequest>,
) -> Result<Json<ApiResponse<UserResponse>>, StatusCode> {
    // First get the current user
    let user = match User::find_by_id(&pool, claims.sub).await {
        Ok(Some(user)) => user,
        Ok(None) => return Err(StatusCode::NOT_FOUND),
        Err(e) => {
            tracing::error!("Failed to get user: {}", e);
            return Err(StatusCode::INTERNAL_SERVER_ERROR);
        }
    };

    // Update the user
    match user.update(&pool, request).await {
        Ok(updated_user) => Ok(Json(ApiResponse::success(updated_user.into()))),
        Err(e) => {
            tracing::error!("Failed to update user: {}", e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}