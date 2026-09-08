use anyhow::Result;
use sqlx::{postgres::PgPoolOptions, PgPool};
use std::time::Duration;

/// Database configuration structure for SQLx PostgreSQL connections
#[derive(Debug, Clone)]
pub struct DatabaseConfig {
    /// Database connection URL
    pub url: String,
    /// Maximum number of connections in the pool
    pub max_connections: u32,
    /// Minimum number of connections in the pool
    pub min_connections: u32,
    /// Timeout for acquiring a connection from the pool
    pub connect_timeout: Duration,
}

impl DatabaseConfig {
    /// Create a new database configuration from environment variables
    pub fn from_env() -> Result<Self> {
        let url = std::env::var("DATABASE_URL")
            .map_err(|_| anyhow::anyhow!("DATABASE_URL environment variable is required"))?;

        // Parse optional configuration from environment variables with defaults
        let max_connections = std::env::var("DB_MAX_CONNECTIONS")
            .unwrap_or_else(|_| "20".to_string())
            .parse::<u32>()
            .unwrap_or(20);

        let min_connections = std::env::var("DB_MIN_CONNECTIONS")
            .unwrap_or_else(|_| "5".to_string())
            .parse::<u32>()
            .unwrap_or(5);

        let connect_timeout_secs = std::env::var("DB_CONNECT_TIMEOUT")
            .unwrap_or_else(|_| "10".to_string())
            .parse::<u64>()
            .unwrap_or(10);

        Ok(Self {
            url,
            max_connections,
            min_connections,
            connect_timeout: Duration::from_secs(connect_timeout_secs),
        })
    }

    /// Create a connection pool with the configured settings
    pub async fn create_pool(&self) -> Result<PgPool> {
        let pool = PgPoolOptions::new()
            .max_connections(self.max_connections)
            .min_connections(self.min_connections)
            .acquire_timeout(self.connect_timeout)
            .connect(&self.url)
            .await?;

        Ok(pool)
    }

    /// Test the database connection
    pub async fn test_connection(&self) -> Result<()> {
        let pool = self.create_pool().await?;
        
        // Simple query to test connectivity
        sqlx::query("SELECT 1")
            .execute(&pool)
            .await
            .map_err(|e| anyhow::anyhow!("Database connection test failed: {}", e))?;

        pool.close().await;
        Ok(())
    }
}