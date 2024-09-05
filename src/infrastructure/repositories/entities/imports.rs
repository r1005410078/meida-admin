use diesel::{
    dsl::{exists, insert_into, select},
    prelude::{Insertable, Queryable},
    update, AsChangeset, ExpressionMethods, QueryDsl, RunQueryDsl, SelectableHelper,
};
use serde::{Deserialize, Serialize};

use crate::{
    infrastructure::{
        db::connection::DBPool,
        repositories::{dao::imports::ImportsPropertiesPO, object_value::query_value::TableData},
    },
    schema::imports_properties,
};

#[derive(Debug, Clone, Serialize, Deserialize, Insertable, AsChangeset, Default)]
#[diesel(table_name = imports_properties)]
pub struct ImportsPropertiesDto {
    pub id: String,
    // 销售类型
    pub pice_type: Option<String>,
    // 用途
    pub usage: Option<String>,
    // 平台
    pub platform: Option<String>,
    // 文件状态
    pub file_status: String,
    // 文件名
    pub file_name: Option<String>,
    // 文件路径
    pub file_path: Option<String>,
    // 文件大小
    pub file_size: Option<i64>,
    // 文件错误
    pub file_error: Option<String>,
}

impl ImportsPropertiesDto {
    pub fn new_error(id: String, error: String) -> Self {
        Self {
            file_error: Some(error),
            file_status: "2".to_string(),
            id,
            ..Default::default()
        }
    }

    pub fn new_success(id: String) -> Self {
        Self {
            file_status: "1".to_string(),
            id,
            ..Default::default()
        }
    }

    pub fn save(&self, db: DBPool) -> Result<(), diesel::result::Error> {
        let conn = &mut db.get().unwrap();
        let existed: bool = select(exists(
            imports_properties::table.filter(imports_properties::id.eq(&self.id)),
        ))
        .get_result(conn)?;

        if existed {
            update(imports_properties::table)
                .filter(imports_properties::id.eq(&self.id))
                .set(self)
                .execute(conn)?;
        } else {
            insert_into(imports_properties::table)
                .values(self)
                .execute(conn)?;
        }

        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QueryImportsPropertiesDto {
    page_size: Option<i64>,
    page_index: Option<i64>,
}

impl QueryImportsPropertiesDto {
    pub fn list(&self, db: DBPool) -> TableData<ImportsPropertiesPO> {
        use crate::schema::imports_properties::dsl::*;
        let conn = &mut db.get().unwrap();

        let total = imports_properties.count().get_result::<i64>(conn).unwrap();

        let page_index = self.page_index.unwrap_or(1);
        let page_size = self.page_size.unwrap_or(10);
        let offset = (page_index - 1) * page_size;

        let data = imports_properties
            .select(ImportsPropertiesPO::as_select())
            .order(updated_at.desc())
            .offset(offset)
            .limit(page_size)
            .load::<ImportsPropertiesPO>(conn)
            .expect("Error loading imports  properties");

        TableData { data, total }
    }
}
