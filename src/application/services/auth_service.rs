use crate::domain::{
    entities::user::User,
    repositories::user_repository::UserRepository,
};
use crate::application::dtos::auth::{LoginDto, RegisterDto};
use bcrypt::{hash, verify, DEFAULT_COST};
use jsonwebtoken::{encode, EncodingKey, Header};
use serde::{Deserialize, Serialize};
use chrono::{Utc, Duration};

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String,
    pub exp: i64,
}

pub struct AuthService {
    user_repository: UserRepository,
    jwt_secret: String,
}

impl AuthService {
    pub fn new(user_repository: UserRepository, jwt_secret: String) -> Self {
        Self {
            user_repository,
            jwt_secret,
        }
    }

    pub async fn register(&self, dto: RegisterDto) -> Result<User, Box<dyn std::error::Error>> {
        // Check if user exists
        if let Some(_) = self.user_repository.find_by_email(&dto.email).await? {
            return Err("User already exists".into());
        }

        // Hash password
        let password_hash = hash(dto.password.as_bytes(), DEFAULT_COST)?;

        // Create user
        let user = User::new(dto.email, password_hash);
        let created_user = self.user_repository.create(user).await?;

        Ok(created_user)
    }

    pub async fn login(&self, dto: LoginDto) -> Result<String, Box<dyn std::error::Error>> {
        // Find user
        let user = self.user_repository
            .find_by_email(&dto.email)
            .await?
            .ok_or("User not found")?;

        // Verify password
        if !verify(dto.password.as_bytes(), &user.password_hash)? {
            return Err("Invalid password".into());
        }

        // Generate JWT
        let expiration = Utc::now()
            .checked_add_signed(Duration::hours(24))
            .expect("valid timestamp")
            .timestamp();

        let claims = Claims {
            sub: user.id.to_string(),
            exp: expiration,
        };

        let token = encode(
            &Header::default(),
            &claims,
            &EncodingKey::from_secret(self.jwt_secret.as_bytes()),
        )?;

        Ok(token)
    }
} 