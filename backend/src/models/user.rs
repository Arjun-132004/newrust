use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;
use validator::Validate;

#[derive(Debug, FromRow, Serialize, Clone)]
pub struct User {
    pub id: Uuid,
    pub email: String,
    #[serde(skip_serializing)]
    pub password_hash: String,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub is_active: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Serialize)]
pub struct UserResponse {
    pub id: Uuid,
    pub email: String,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub is_active: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl From<User> for UserResponse {
    fn from(user: User) -> Self {
        Self {
            id: user.id,
            email: user.email,
            first_name: user.first_name,
            last_name: user.last_name,
            is_active: user.is_active,
            created_at: user.created_at,
            updated_at: user.updated_at,
        }
    }
}

#[derive(Debug, Deserialize, Validate)]
pub struct CreateUserRequest {
    #[validate(email(message = "Please provide a valid email address"))]
    pub email: String,
    
    #[validate(length(min = 8, message = "Password must be at least 8 characters long"))]
    pub password: String,
    
    #[validate(length(min = 1, message = "First name cannot be empty"))]
    pub first_name: Option<String>,
    
    pub last_name: Option<String>,
}

#[derive(Debug, Deserialize, Validate)]
pub struct LoginRequest {
    #[validate(email(message = "Please provide a valid email address"))]
    pub email: String,
    
    #[validate(length(min = 1, message = "Password is required"))]
    pub password: String,
}

#[derive(Debug, Deserialize, Validate)]
pub struct UpdateUserRequest {
    pub first_name: Option<String>,
    pub last_name: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct LoginResponse {
    pub user: UserResponse,
    pub token: String,
    pub expires_at: DateTime<Utc>,
}

impl User {
    pub async fn create(pool: &sqlx::PgPool, request: CreateUserRequest) -> Result<Self, sqlx::Error> {
        let password_hash = bcrypt::hash(&request.password, bcrypt::DEFAULT_COST)
            .map_err(|_| sqlx::Error::Protocol("Failed to hash password".to_string()))?;

        let user = sqlx::query_as!(
            User,
            r#"
            INSERT INTO users (email, password_hash, first_name, last_name)
            VALUES ($1, $2, $3, $4)
            RETURNING 
                id, 
                email, 
                password_hash, 
                first_name, 
                last_name, 
                is_active as "is_active!: bool", 
                created_at as "created_at!: DateTime<Utc>", 
                updated_at as "updated_at!: DateTime<Utc>"
            "#,
            request.email,
            password_hash,
            request.first_name,
            request.last_name
        )
        .fetch_one(pool)
        .await?;

        Ok(user)
    }

    pub async fn find_by_email(pool: &sqlx::PgPool, email: &str) -> Result<Option<Self>, sqlx::Error> {
        let user = sqlx::query_as!(
            User,
            r#"
            SELECT 
                id, 
                email, 
                password_hash, 
                first_name, 
                last_name, 
                is_active as "is_active!: bool", 
                created_at as "created_at!: DateTime<Utc>", 
                updated_at as "updated_at!: DateTime<Utc>" 
            FROM users 
            WHERE email = $1 AND is_active = true
            "#,
            email
        )
        .fetch_optional(pool)
        .await?;

        Ok(user)
    }

    pub async fn find_by_id(pool: &sqlx::PgPool, id: Uuid) -> Result<Option<Self>, sqlx::Error> {
        let user = sqlx::query_as!(
            User,
            r#"
            SELECT 
                id, 
                email, 
                password_hash, 
                first_name, 
                last_name, 
                is_active as "is_active!: bool", 
                created_at as "created_at!: DateTime<Utc>", 
                updated_at as "updated_at!: DateTime<Utc>" 
            FROM users 
            WHERE id = $1 AND is_active = true
            "#,
            id
        )
        .fetch_optional(pool)
        .await?;

        Ok(user)
    }

    pub async fn update(&self, pool: &sqlx::PgPool, request: UpdateUserRequest) -> Result<Self, sqlx::Error> {
        let user = sqlx::query_as!(
            User,
            r#"
            UPDATE users 
            SET first_name = $1, last_name = $2, updated_at = NOW()
            WHERE id = $3
            RETURNING 
                id, 
                email, 
                password_hash, 
                first_name, 
                last_name, 
                is_active as "is_active!: bool", 
                created_at as "created_at!: DateTime<Utc>", 
                updated_at as "updated_at!: DateTime<Utc>"
            "#,
            request.first_name,
            request.last_name,
            self.id
        )
        .fetch_one(pool)
        .await?;

        Ok(user)
    }

    pub fn verify_password(&self, password: &str) -> bool {
        bcrypt::verify(password, &self.password_hash).unwrap_or(false)
    }
}