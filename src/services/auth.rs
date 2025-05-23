use argon2::{
    password_hash::{rand_core::OsRng, PasswordHash, PasswordHasher, PasswordVerifier, SaltString},
    Argon2,
};
use jsonwebtoken::{encode, EncodingKey, Header};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use chrono::{Duration, Utc};
use anyhow::{Result, anyhow};

use crate::models::user::{CreateUserDto, LoginDto, User};

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String,  // user id
    pub exp: i64,     // expiration time
    pub email: String,
}

#[derive(Clone)]
pub struct AuthService {
    pool: PgPool,
    jwt_secret: String,
}

impl AuthService {
    pub fn new(pool: PgPool, jwt_secret: String) -> Self {
        Self { pool, jwt_secret }
    }

    pub async fn register(&self, dto: CreateUserDto) -> Result<User> {
        // Check if user already exists
        if let Some(_) = User::find_by_email(&self.pool, &dto.email).await? {
            return Err(anyhow!("User with this email already exists"));
        }

        // Hash password
        let salt = SaltString::generate(&mut OsRng);
        let argon2 = Argon2::default();
        let password_hash = argon2
            .hash_password(dto.password.as_bytes(), &salt)
            .map_err(|e| anyhow!("Password hashing error: {}", e))?
            .to_string();

        // Create user
        let user = User::create(&self.pool, dto, password_hash).await?;
        Ok(user)
    }

    pub async fn login(&self, dto: LoginDto) -> Result<(String, User)> {
        // Find user
        let user = User::find_by_email(&self.pool, &dto.email)
            .await?
            .ok_or_else(|| anyhow!("Invalid credentials"))?;

        // Verify password
        let parsed_hash = PasswordHash::new(&user.password_hash)
            .map_err(|e| anyhow!("Password hash error: {}", e))?;
        
        if !Argon2::default()
            .verify_password(dto.password.as_bytes(), &parsed_hash)
            .is_ok()
        {
            return Err(anyhow!("Invalid credentials"));
        }

        // Generate JWT
        let expiration = Utc::now()
            .checked_add_signed(Duration::hours(24))
            .unwrap()
            .timestamp();

        let claims = Claims {
            sub: user.id.to_string(),
            exp: expiration,
            email: user.email.clone(),
        };

        let token = encode(
            &Header::default(),
            &claims,
            &EncodingKey::from_secret(self.jwt_secret.as_bytes()),
        )?;

        Ok((token, user))
    }
} 