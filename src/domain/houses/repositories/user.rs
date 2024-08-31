use async_trait::async_trait;

use crate::infrastructure::repositories::entities::users::UsersVO;

#[async_trait]
pub trait UserRepository {
    async fn get_user_by_token(&self, token: &str) -> Option<UsersVO>;
}
