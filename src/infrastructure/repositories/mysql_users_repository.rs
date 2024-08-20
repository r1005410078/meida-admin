use super::{
    dao::users::{QueryUsersDao, SaveUsersDao},
    entities::users::{UsersPO, UsersVO},
    object_value::query_value::TableData,
};
use crate::{
    common::jwt::Claims,
    infrastructure::db::connection::{establish_connection, DBPool},
};
use diesel::{ExpressionMethods, OptionalExtension, QueryDsl, RunQueryDsl, SelectableHelper};

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

    pub async fn find_by_username<'a>(&self, input_username: &str) -> Option<UsersPO> {
        use crate::schema::users::dsl::*;
        let conn = &mut self.pool.get().unwrap();
        users
            .filter(username.eq(input_username))
            .first::<UsersPO>(conn)
            .optional()
            .expect("Error loading users")
    }

    pub async fn list<'a>(&self, query: &QueryUsersDao<'a>) -> TableData<UsersVO> {
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

    pub async fn get_user(&self, input_id: &str) -> Option<UsersVO> {
        use crate::schema::users::dsl::*;
        let mut conn = self.pool.get().expect("Error loading houses");
        users
            .select(UsersVO::as_select())
            .filter(id.eq(input_id))
            .first::<UsersVO>(&mut conn)
            .optional()
            .expect("Error loading users")
    }
}
