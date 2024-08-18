use serde::{Deserialize, Serialize};

use crate::infrastructure::repositories::dao::users::{LoginDao, SaveUsersDao};

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
    password_hash: Option<String>,
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
            password_hash: self.password_hash.as_deref(),
            phone: self.phone.as_deref(),
            avatar: self.avatar.as_deref(),
            is_active: self.is_active,
            role: self.role.as_deref(),
        }
    }
}
