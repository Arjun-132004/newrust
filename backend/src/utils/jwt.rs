use chrono::{DateTime, Duration, Utc};
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JwtClaims {
    pub sub: Uuid,      // Subject (user id)
    pub email: String,  // Email
    pub exp: DateTime<Utc>, // Expiration time
    pub iat: DateTime<Utc>, // Issued at
}

impl JwtClaims {
    pub fn new(user_id: Uuid, email: String) -> Self {
        let now = Utc::now();
        let expiration = now + Duration::hours(24); // 24 hours from now

        Self {
            sub: user_id,
            email,
            exp: expiration,
            iat: now,
        }
    }

    pub fn encode(&self) -> Result<String, jsonwebtoken::errors::Error> {
        let secret = std::env::var("JWT_SECRET")
            .unwrap_or_else(|_| "default-secret-change-this".to_string());
        
        encode(
            &Header::default(),
            self,
            &EncodingKey::from_secret(secret.as_ref()),
        )
    }

    pub fn decode(token: &str) -> Result<Self, jsonwebtoken::errors::Error> {
        let secret = std::env::var("JWT_SECRET")
            .unwrap_or_else(|_| "default-secret-change-this".to_string());

        let token_data = decode::<JwtClaims>(
            token,
            &DecodingKey::from_secret(secret.as_ref()),
            &Validation::default(),
        )?;

        Ok(token_data.claims)
    }

    pub fn is_expired(&self) -> bool {
        Utc::now() > self.exp
    }
}