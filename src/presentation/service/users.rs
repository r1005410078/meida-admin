use std::sync::Arc;

use crate::infrastructure::repositories::{
    dao::users::{LoginDao, QueryUsersDao, SaveUsersDao},
    entities::users::UsersPO,
    mysql_users_repository::MysqlUsersRepository,
    object_value::query_value::TableData,
};

pub struct UsersService {
    repo: Arc<MysqlUsersRepository>,
}

impl UsersService {
    pub fn new(repo: Arc<MysqlUsersRepository>) -> Self {
        Self { repo }
    }

    pub async fn save<'a>(&self, event: &SaveUsersDao<'a>) -> Result<(), diesel::result::Error> {
        self.repo.save_user(event).await
    }

    pub async fn login<'a>(&self, user: &LoginDao<'a>) -> anyhow::Result<String> {
        self.repo.login(user).await
    }

    pub async fn list<'a>(&self, user: &QueryUsersDao<'a>) -> TableData<UsersPO> {
        self.repo.list(user).await
    }

    pub async fn delete_user(&self, input_id: &str) -> Result<(), diesel::result::Error> {
        self.repo.delete_user(input_id).await
    }
}
