use std::sync::Arc;

use crate::infrastructure::repositories::{
    dao::users::{LoginDao, SaveUsersDao},
    mysql_users_repository::MysqlUsersRepository,
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
}
