use chrono::NaiveDateTime;
use diesel::{prelude::Queryable, Selectable};
use serde::{Deserialize, Serialize};

use crate::schema::users;

#[derive(Debug, Clone, Serialize, Deserialize, Queryable, Selectable)]
#[diesel(table_name = users)]
pub struct UsersPO {
    pub id: String,
    pub username: String,
    pub password_hash: String,
    pub phone: String,
    pub avatar: Option<String>,
    pub is_active: Option<bool>,
    pub role: Option<String>,
    pub created_at: Option<NaiveDateTime>,
    pub updated_at: Option<NaiveDateTime>,
}
