use std::sync::Arc;

use async_trait::async_trait;

use crate::{
    common::event_channel::EventHandler,
    domain::houses::events::rental_house::SaveRentalHouseEvent,
    infrastructure::repositories::mysql_house_repository::MysqlHouseRepository,
    presentation::service::house::HouseService,
};

pub struct RentalHouseHandler {
    service: HouseService,
}

impl RentalHouseHandler {
    pub fn new(reo: Arc<MysqlHouseRepository>) -> Self {
        Self {
            service: HouseService::new(reo),
        }
    }
}

#[async_trait]
impl EventHandler<SaveRentalHouseEvent> for RentalHouseHandler {
    async fn handle(&self, event: SaveRentalHouseEvent) -> anyhow::Result<()> {
        self.service.save_rental_house(event).await?;
        Ok(())
    }
}