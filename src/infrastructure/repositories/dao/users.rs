use crate::{
    infrastructure::{
        db::connection::DBPool,
        repositories::{entities::users::UsersPO, object_value::query_value::TableData},
    },
    schema::users,
};
use diesel::{
    prelude::{AsChangeset, Insertable},
    ExpressionMethods, OptionalExtension, QueryDsl, RunQueryDsl, SelectableHelper,
    TextExpressionMethods,
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

pub struct QueryUsersDao<'a> {
    pub username: Option<&'a str>,
    pub phone: Option<&'a str>,
    pub is_active: Option<bool>,
    pub role: Option<&'a str>,
    pub page_index: Option<i64>,
    pub page_size: Option<i64>,
}

impl QueryUsersDao<'_> {
    pub fn list(&self, pool: DBPool) -> TableData<UsersPO> {
        use crate::schema::users::dsl::*;

        let conn = &mut pool.get().unwrap();

        let get_query = || {
            let mut result = users.into_boxed();

            if let Some(ref input_is_active) = self.is_active {
                result = result.filter(is_active.eq(input_is_active));
            }

            if let Some(ref input_phone) = self.phone {
                result = result.filter(phone.like(format!("%{}%", input_phone)));
            }

            if let Some(ref input_username) = self.username {
                result = result.filter(username.like(format!("%{}%", input_username)));
            }

            if let Some(ref input_role) = self.role {
                result = result.filter(role.eq(input_role));
            }

            result
        };

        let total = get_query()
            .count()
            .get_result(conn)
            .expect("Error loading users");

        let page_index = self.page_index.unwrap_or(1);
        let page_size = self.page_size.unwrap_or(10);

        let result = get_query()
            .offset((page_index - 1) * page_size)
            .limit(page_size);

        let data = result.load::<UsersPO>(conn).expect("Error loading houses");

        TableData::new(data, total)
    }
}
