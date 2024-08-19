use serde::{Deserialize, Serialize};

use crate::infrastructure::repositories::dao::users::{LoginDao, QueryUsersDao, SaveUsersDao};

#[derive(Debug, Serialize, Deserialize)]
pub struct LoginDto {
    pub username: String,
    pub password: String,
}

impl LoginDto {
    pub fn convert_to_dao(&self) -> LoginDao {
        LoginDao {
            username: &self.username,
            password_hash: &self.password,
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SaveUsersDto {
    id: Option<String>,
    username: Option<String>,
    password: Option<String>,
    phone: Option<String>,
    avatar: Option<String>,
    is_active: Option<bool>,
    role: Option<String>,
}

impl SaveUsersDto {
    pub fn convert_to_dao(&self) -> SaveUsersDao {
        SaveUsersDao {
            id: self.id.as_deref(),
            username: self.username.as_deref(),
            password_hash: self.password.as_deref(),
            phone: self.phone.as_deref(),
            avatar: self.avatar.as_deref(),
            is_active: self.is_active,
            role: self.role.as_deref(),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct QueryUsersDto {
    pub username: Option<String>,
    pub phone: Option<String>,
    pub is_active: Option<bool>,
    pub role: Option<String>,
    pub page_index: Option<i64>,
    pub page_size: Option<i64>,
}

impl QueryUsersDto {
    pub fn convert_to_dao(&self) -> QueryUsersDao {
        QueryUsersDao {
            username: self.username.as_deref(),
            phone: self.phone.as_deref(),
            is_active: self.is_active,
            role: self.role.as_deref(),
            page_index: self.page_index,
            page_size: self.page_size,
        }
    }
}
