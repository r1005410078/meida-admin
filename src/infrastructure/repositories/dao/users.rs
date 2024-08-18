use crate::{
    infrastructure::{db::connection::DBPool, repositories::entities::users::UsersPO},
    schema::users,
};
use diesel::{
    prelude::{AsChangeset, Insertable},
    ExpressionMethods, OptionalExtension, QueryDsl, RunQueryDsl, SelectableHelper,
};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, Insertable, AsChangeset)]
#[diesel(table_name = users)]
pub struct SaveUsersDao<'a> {
    pub id: Option<&'a str>,
    pub username: Option<&'a str>,
    pub password_hash: Option<&'a str>,
    pub phone: Option<&'a str>,
    pub avatar: Option<&'a str>,
    pub is_active: Option<bool>,
    pub role: Option<&'a str>,
}

impl SaveUsersDao<'_> {
    pub fn save(&self, pool: DBPool) -> Result<(), diesel::result::Error> {
        let conn = &mut pool.get().unwrap();

        use crate::schema::users::dsl::*;

        if let Some(update_id) = self.id {
            diesel::update(users.filter(id.eq(update_id)))
                .set(self)
                .execute(conn)?;
        } else {
            diesel::insert_into(users).values(self).execute(conn)?;
        }

        Ok(())
    }
}

pub struct LoginDao<'a> {
    pub username: &'a str,
    pub password_hash: &'a str,
}

impl LoginDao<'_> {
    pub fn user_exists(&self, pool: DBPool) -> Option<UsersPO> {
        use crate::schema::users::dsl::*;
        let conn = &mut pool.get().unwrap();
        users
            .filter(username.eq(self.username))
            .filter(password_hash.eq(self.password_hash))
            .select(UsersPO::as_select())
            .first::<UsersPO>(conn)
            .optional()
            .expect("Error loading users")
    }
}
