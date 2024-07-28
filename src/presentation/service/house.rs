use std::sync::Arc;

use crate::{
    domain::houses::{
        entities::house::HousePO,
        events::house::{NewHouseEvent, UpdateHouseEvent},
    },
    infrastructure::repositories::{
        dao::house::NewHouseDto, mysql_house_repository::MysqlHouseRepository,
    },
};

pub struct HouseService {
    repo: Arc<MysqlHouseRepository>,
}

impl HouseService {
    pub fn new(repo: Arc<MysqlHouseRepository>) -> Self {
        Self { repo }
    }

    pub async fn create(&self, event: NewHouseEvent) -> Result<(), diesel::result::Error> {
        self.repo.generate_house_event_record(event.into()).await
    }

    pub async fn update(&self, event: UpdateHouseEvent) -> Result<(), diesel::result::Error> {
        if let Some(house) = self.repo.get_by_house_id(event.house_id.clone()).await {
            self.repo
                .generate_house_event_record(NewHouseDto::new_by_event(event, house))
                .await
        } else {
            Ok(())
        }
    }

    pub async fn list(&self) -> Vec<HousePO> {
        self.repo.list().await
    }
}
