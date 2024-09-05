use chrono::NaiveDateTime;
use diesel::{Queryable, Selectable};
use serde::{Deserialize, Serialize};

use crate::schema::imports_properties;

#[derive(Debug, Clone, Serialize, Deserialize, Selectable, Queryable)]
#[diesel(table_name = imports_properties)]
pub struct ImportsPropertiesPO {
    pub id: String,
    // 销售类型
    pub pice_type: String,
    // 用途
    pub usage: String,
    // 平台
    pub platform: String,
    // 文件状态
    pub file_status: String,
    // 文件名
    pub file_name: String,
    // 文件路径
    pub file_path: String,
    // 文件大小
    pub file_size: i64,
    // 错误信息
    pub file_error: Option<String>,
    // 创建时间
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}
