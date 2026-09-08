use crate::models::{ApiResponse, CreateUserRequest, LoginRequest};
use crate::services::AuthService;
use axum::{extract::State, http::StatusCode, Json};
use sqlx::PgPool;

pub async fn register(
    State(pool): State<PgPool>,
    Json(request): Json<CreateUserRequest>,
) -> Result<Json<ApiResponse<crate::models::UserResponse>>, StatusCode> {
    match AuthService::register(&pool, request).await {
        Ok(user) => Ok(Json(ApiResponse::success(user))),
        Err(e) => {
            tracing::error!("Registration failed: {}", e);
            Err(StatusCode::BAD_REQUEST)
        }
    }
}

pub async fn login(
    State(pool): State<PgPool>,
    Json(request): Json<LoginRequest>,
) -> Result<Json<ApiResponse<crate::models::LoginResponse>>, StatusCode> {
    match AuthService::login(&pool, request).await {
        Ok(response) => Ok(Json(ApiResponse::success(response))),
        Err(e) => {
            tracing::error!("Login failed: {}", e);
            Err(StatusCode::UNAUTHORIZED)
        }
    }
}