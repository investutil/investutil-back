use async_trait::async_trait;
use sqlx::PgPool;
use crate::domain::{
    entities::user::User,
    repositories::user_repository::UserRepositoryTrait,
};

pub struct PgUserRepository {
    pool: PgPool,
}

impl PgUserRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl UserRepositoryTrait for PgUserRepository {
    async fn find_by_email(&self, email: &str) -> Result<Option<User>, Box<dyn std::error::Error>> {
        let user = sqlx::query_as!(
            User,
            r#"
            SELECT id, email, password_hash, created_at, updated_at
            FROM users
            WHERE email = $1
            "#,
            email
        )
        .fetch_optional(&self.pool)
        .await?;

        Ok(user)
    }

    async fn create(&self, user: User) -> Result<User, Box<dyn std::error::Error>> {
        let created_user = sqlx::query_as!(
            User,
            r#"
            INSERT INTO users (id, email, password_hash, created_at, updated_at)
            VALUES ($1, $2, $3, $4, $5)
            RETURNING id, email, password_hash, created_at, updated_at
            "#,
            user.id,
            user.email,
            user.password_hash,
            user.created_at,
            user.updated_at
        )
        .fetch_one(&self.pool)
        .await?;

        Ok(created_user)
    }

    async fn update(&self, user: &User) -> Result<User, Box<dyn std::error::Error>> {
        let updated_user = sqlx::query_as!(
            User,
            r#"
            UPDATE users
            SET password_hash = $1, updated_at = $2
            WHERE id = $3
            RETURNING id, email, password_hash, created_at, updated_at
            "#,
            user.password_hash,
            user.updated_at,
            user.id
        )
        .fetch_one(&self.pool)
        .await?;

        Ok(updated_user)
    }
} 