use sqlx::sqlite::{SqlitePool, SqliteConnectOptions};
use std::str::FromStr;

pub struct Database {
    pool: SqlitePool,
}

impl Database {
    pub async fn new(db_url: &str, passphrase: &str) -> Result<Self, sqlx::Error> {
        // Configure connection with encryption (PRAGMA key)
        let options = SqliteConnectOptions::from_str(db_url)?
            .create_if_missing(true);
        
        // In a real implementation, the passphrase would be securely managed.
        // SQLCipher uses 'PRAGMA key' to encrypt/decrypt the database.
        let pool = SqlitePool::connect_with(options).await?;
        
        // Apply encryption key
        sqlx::query(&format!("PRAGMA key = '{}';", passphrase))
            .execute(&pool)
            .await?;

        Ok(Self { pool })
    }

    /// Safely updates database using parameterized queries.
    #[allow(dead_code)]
    pub async fn update_hits_safe(&self, process_path: &str) -> Result<(), sqlx::Error> {
        sqlx::query("UPDATE connections SET hits = hits + 1 WHERE process = ?")
            .bind(process_path)
            .execute(&self.pool)
            .await?;
            
        Ok(())
    }
}