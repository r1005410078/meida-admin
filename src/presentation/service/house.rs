use std::sync::Arc;

use crate::{
    domain::houses::{entities::house::HousePO, events::house::SaveHouseEvent},
    infrastructure::repositories::{
        dao::house::QueryHouseDao, mysql_house_repository::MysqlHouseRepository,
        object_value::query_value::TableData,
    },
};

pub struct HouseService {
    repo: Arc<MysqlHouseRepository>,
}

impl HouseService {
    pub fn new(repo: Arc<MysqlHouseRepository>) -> Self {
        Self { repo }
    }

    pub async fn save(&self, event: SaveHouseEvent) -> anyhow::Result<()> {
        self.repo.save_house(event).await
    }

    pub async fn delete(&self, house_id: String) -> anyhow::Result<()> {
        self.repo.delete(house_id).await
    }

    pub async fn list(&self, query: QueryHouseDao) -> TableData<HousePO> {
        self.repo.list(query).await
    }

    pub async fn get_by_house_id(&self, input_house_id: String) -> Option<HousePO> {
        self.repo.get_by_house_id(input_house_id).await
    }

    pub async fn list_by_owner_name(&self, input_owner_name: String) -> Vec<HousePO> {
        self.repo.list_by_owner_name(input_owner_name).await
    }
}
