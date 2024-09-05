use actix_multipart::form::{tempfile::TempFile, text::Text, MultipartForm};
use anyhow::anyhow;
use std::{env, sync::Arc};
use uuid::Uuid;

use crate::{
    common::{excel::ReadHouseExcel, rm_imports_properties::rm_imports_properties},
    infrastructure::repositories::{
        dao::imports::ImportsPropertiesPO,
        entities::imports::{ImportsPropertiesDto, QueryImportsPropertiesDto},
        mysql_house_repository::MysqlHouseRepository,
        object_value::query_value::TableData,
    },
};

pub struct ImportsService {
    repo: Arc<MysqlHouseRepository>,
}

#[derive(Debug, MultipartForm)]
pub struct UploadForm {
    #[multipart(limit = "100MB")]
    file: TempFile,
    usage: Option<Text<i32>>,
    pice_type: Option<Text<i32>>,
    platform: Option<Text<i32>>,
}

impl ImportsService {
    pub fn new(repo: Arc<MysqlHouseRepository>) -> Self {
        Self { repo: repo }
    }

    pub async fn upload_file(
        &self,
        MultipartForm(form): MultipartForm<UploadForm>,
    ) -> anyhow::Result<()> {
        let file_name = form.file.file_name.unwrap();
        let uuid = Uuid::new_v4().to_string();
        let upload_dir = env::var("UPLOAD_DIR").expect("UPLOAD_DIR must be set");
        let file_path = format!("{}/{}+{}", upload_dir, uuid.clone(), file_name.clone());

        form.file.file.persist_noclobber(file_path.clone())?;
        // 保存到数据库
        self.repo.save_imports_properties(ImportsPropertiesDto {
            id: uuid,
            pice_type: form.pice_type.map(|v| v.to_string()),
            usage: form.usage.map(|v| v.to_string()),
            platform: form.platform.map(|v| v.to_string()),
            file_status: "-1".to_owned(),
            file_name: Some(file_name),
            file_path: Some(file_path),
            file_size: Some(form.file.size as i64),
            file_error: None,
        })?;

        Ok(())
    }

    // 同步excel
    pub async fn sync_excel(&self) -> anyhow::Result<()> {
        let data = self.repo.sync_imports_properties();

        for item in data {
            match ReadHouseExcel::read_xlsx(&item.file_path)
                .map_err(|e| anyhow!("读取 excel 错误: {}", e))
            {
                Ok(data) => {
                    println!("同步成功: {}", item.id);
                    self.repo
                        .save_imports_properties(ImportsPropertiesDto::new_success(
                            item.id.clone(),
                        ))?;
                }
                Err(e) => {
                    self.repo
                        .save_imports_properties(ImportsPropertiesDto::new_error(
                            item.id.clone(),
                            e.to_string(),
                        ))?;
                }
            }
        }

        Ok(())
    }

    pub async fn list_imports_properties(
        &self,
        dto: QueryImportsPropertiesDto,
    ) -> TableData<ImportsPropertiesPO> {
        self.repo.query_imports_properties(dto)
    }

    // 删除
    pub async fn delete_imports_properties(
        &self,
        ids: Vec<String>,
    ) -> Result<(), diesel::result::Error> {
        self.repo.delete_imports_properties(ids.clone()).await?;

        // 删除文件
        for ref id in ids {
            rm_imports_properties(&id);
        }

        Ok(())
    }
}
