use std::sync::Arc;

use crate::{
    common::{argon::Argon, jwt::Claims},
    infrastructure::repositories::{
        dao::users::{LoginDao, QueryUsersDao, SaveUsersDao},
        entities::users::UsersVO,
        mysql_users_repository::MysqlUsersRepository,
        object_value::query_value::TableData,
    },
};

pub struct UsersService {
    repo: Arc<MysqlUsersRepository>,
}

impl UsersService {
    pub fn new(repo: Arc<MysqlUsersRepository>) -> Self {
        Self { repo }
    }

    pub async fn save<'a>(
        &self,
        event: &mut SaveUsersDao<'a>,
    ) -> Result<(), diesel::result::Error> {
        event.password_hash = if let Some(ref password) = event.password_hash {
            Some(Argon::password_hash(password))
        } else {
            None
        };

        self.repo.save_user(event).await
    }

    pub async fn login<'a>(&self, input_user: &LoginDao<'a>) -> anyhow::Result<String> {
        if let Some(user) = self.repo.find_by_username(&input_user.username).await? {
            if Argon::verify_password(input_user.password_hash, &user.password_hash) {
                if Some(true) == user.is_active {
                    let token = Claims::new(
                        user.id.to_string(),
                        user.username.to_string(),
                        user.role.clone().unwrap_or("admin".to_string()),
                    )
                    .get_token();

                    Ok(token)
                } else {
                    Err(anyhow::anyhow!("用户已被禁用"))
                }
            } else {
                Err(anyhow::anyhow!("用户名或密码错误"))
            }
        } else {
            Err(anyhow::anyhow!("用户名或密码错误"))
        }
    }

    pub async fn list<'a>(&self, user: &QueryUsersDao<'a>) -> TableData<UsersVO> {
        self.repo.list(user).await
    }

    pub async fn delete_user(&self, input_id: &str) -> Result<(), diesel::result::Error> {
        self.repo.delete_user(input_id).await
    }

    pub async fn get_user(&self, input_id: &str) -> Option<UsersVO> {
        self.repo.get_user(input_id).await
    }
}
