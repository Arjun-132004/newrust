use crate::models::{CreateUserRequest, LoginRequest, LoginResponse, User, UserResponse};
use crate::utils::jwt::JwtClaims;
use anyhow::{anyhow, Result};
use sqlx::PgPool;
// use uuid::Uuid;
use validator::Validate;

pub struct AuthService;

impl AuthService {
    pub async fn register(pool: &PgPool, request: CreateUserRequest) -> Result<UserResponse> {
        // Validate request
        request.validate()
            .map_err(|e| anyhow!("Validation error: {}", e))?;

        // Check if user already exists
        if User::find_by_email(pool, &request.email).await?.is_some() {
            return Err(anyhow!("User with this email already exists"));
        }

        // Create user
        let user = User::create(pool, request).await
            .map_err(|e| anyhow!("Failed to create user: {}", e))?;

        Ok(user.into())
    }

    pub async fn login(pool: &PgPool, request: LoginRequest) -> Result<LoginResponse> {
        // Validate request
        request.validate()
            .map_err(|e| anyhow!("Validation error: {}", e))?;

        // Find user by email
        let user = User::find_by_email(pool, &request.email)
            .await
            .map_err(|e| anyhow!("Database error: {}", e))?
            .ok_or_else(|| anyhow!("Invalid email or password"))?;

        // Verify password
        if !user.verify_password(&request.password) {
            return Err(anyhow!("Invalid email or password"));
        }

        // Generate JWT token
        let claims = JwtClaims::new(user.id, user.email.clone());
        let token = claims.encode()
            .map_err(|e| anyhow!("Failed to generate token: {}", e))?;

        Ok(LoginResponse {
            user: user.into(),
            token,
            expires_at: claims.exp,
        })
    }

    // pub async fn get_user_by_id(pool: &PgPool, user_id: Uuid) -> Result<UserResponse> {
    //     let user = User::find_by_id(pool, user_id)
    //         .await
    //         .map_err(|e| anyhow!("Database error: {}", e))?
    //         .ok_or_else(|| anyhow!("User not found"))?;

    //     Ok(user.into())
    // }
}