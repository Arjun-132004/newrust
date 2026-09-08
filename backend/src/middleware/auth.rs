use crate::utils::jwt::JwtClaims;
use axum::{
    extract::{Request, State},
    http::{header, StatusCode},
    middleware::Next,
    response::Response,
};
use sqlx::PgPool;

pub async fn auth_middleware(
    State(pool): State<PgPool>,
    mut request: Request,
    next: Next,
) -> Result<Response, StatusCode> {
    // Skip auth for public routes
    let path = request.uri().path();
    if is_public_route(path) {
        return Ok(next.run(request).await);
    }

    // Extract authorization header
    let auth_header = request
        .headers()
        .get(header::AUTHORIZATION)
        .and_then(|header| header.to_str().ok())
        .and_then(|header| header.strip_prefix("Bearer "));

    let token = match auth_header {
        Some(token) => token,
        None => return Err(StatusCode::UNAUTHORIZED),
    };

    // Decode and validate JWT
    let claims = match JwtClaims::decode(token) {
        Ok(claims) => {
            if claims.is_expired() {
                return Err(StatusCode::UNAUTHORIZED);
            }
            claims
        }
        Err(_) => return Err(StatusCode::UNAUTHORIZED),
    };

    // Verify user exists and is active
    match crate::models::User::find_by_id(&pool, claims.sub).await {
        Ok(Some(_)) => {
            // Add user info to request extensions
            request.extensions_mut().insert(claims);
            Ok(next.run(request).await)
        }
        Ok(None) => Err(StatusCode::UNAUTHORIZED),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

fn is_public_route(path: &str) -> bool {
    matches!(
        path,
        "/api/health" | "/api/auth/login" | "/api/auth/register"
    )
}