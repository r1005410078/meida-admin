use crate::{
    common::jwt::Claims,
    infrastructure::db::connection::{establish_connection, DBPool},
};
use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl};

use super::{
    dao::users::{LoginDao, QueryUsersDao, SaveUsersDao},
    entities::users::UsersPO,
    object_value::query_value::TableData,
};

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
            if Some(true) == user.is_active {
                let token = Claims::new(
                    user.username.to_string(),
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
    }

    pub async fn list<'a>(&self, query: &QueryUsersDao<'a>) -> TableData<UsersPO> {
        query.list(self.pool.clone())
    }

    pub fn validate(&self, token: &str) -> anyhow::Result<Claims> {
        Claims::validate(token)
    }

    pub async fn delete_user(&self, input_id: &str) -> Result<(), diesel::result::Error> {
        use crate::schema::users::dsl::*;
        let mut conn = self.pool.get().expect("Error loading houses");
        diesel::delete(users.filter(id.eq(input_id))).execute(&mut conn)?;
        Ok(())
    }
}
