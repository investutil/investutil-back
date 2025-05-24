use async_trait::async_trait;
use crate::domain::entities::user::User;
use std::sync::Arc;

#[async_trait]
pub trait UserRepositoryTrait: Send + Sync {
    async fn find_by_email(&self, email: &str) -> Result<Option<User>, Box<dyn std::error::Error>>;
    async fn create(&self, user: User) -> Result<User, Box<dyn std::error::Error>>;
    async fn update(&self, user: &User) -> Result<User, Box<dyn std::error::Error>>;
}

// 定义一个类型别名，使用 Arc 包装 trait 对象
pub type UserRepository = Arc<dyn UserRepositoryTrait + Send + Sync>; 