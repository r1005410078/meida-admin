use crate::{
    common::jwt::Claims,
    infrastructure::db::connection::{establish_connection, DBPool},
};

use super::dao::users::{LoginDao, SaveUsersDao};

#[derive(Debug)]
pub struct MysqlUsersRepository {
    pub pool: DBPool,
}

impl MysqlUsersRepository {
    pub fn new() -> Self {
        let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
        MysqlUsersRepository {
            pool: establish_connection(&database_url),
        }
    }

    pub async fn save_user<'a>(&self, dto: &SaveUsersDao<'a>) -> Result<(), diesel::result::Error> {
        dto.save(self.pool.clone())
    }

    pub async fn login<'a>(&self, user: &LoginDao<'a>) -> anyhow::Result<String> {
        if let Some(user) = user.user_exists(self.pool.clone()) {
            // Claims::new(user.username.to_string(), result.role.to_string()).encode()

            let token = Claims::new(
                user.username.to_string(),
                user.username.to_string(),
                user.role.clone().unwrap_or("admin".to_string()),
            )
            .get_token();

            Ok(token)
        } else {
            Err(anyhow::anyhow!("用户名或密码错误"))
        }
    }

    pub fn validate(&self, token: &str) -> anyhow::Result<Claims> {
        Claims::validate(token)
    }
}
