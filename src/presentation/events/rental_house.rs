use std::sync::Arc;

use async_trait::async_trait;

use crate::{
    common::event_channel::EventHandler,
    domain::houses::events::rental_house::{
        RentalHouseListedEvent, RentalHouseSoldEvent, RentalHouseUnListedEvent,
        SaveRentalHouseEvent,
    },
    infrastructure::repositories::mysql_house_repository::MysqlHouseRepository,
    presentation::service::rental_house::RentalHouseService,
};

pub struct RentalHouseHandler {
    service: RentalHouseService,
}

impl RentalHouseHandler {
    pub fn new(reo: Arc<MysqlHouseRepository>) -> Self {
        Self {
            service: RentalHouseService::new(reo),
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

#[async_trait]
impl EventHandler<RentalHouseListedEvent> for RentalHouseHandler {
    async fn handle(&self, event: RentalHouseListedEvent) -> anyhow::Result<()> {
        self.service.listed(event).await?;
        Ok(())
    }
}

#[async_trait]
impl EventHandler<RentalHouseUnListedEvent> for RentalHouseHandler {
    async fn handle(&self, event: RentalHouseUnListedEvent) -> anyhow::Result<()> {
        self.service.unlisted(event).await?;
        Ok(())
    }
}

// 出租房租出
#[async_trait]
impl EventHandler<RentalHouseSoldEvent> for RentalHouseHandler {
    async fn handle(&self, event: RentalHouseSoldEvent) -> anyhow::Result<()> {
        self.service.sold(event.clone()).await?;
        Ok(())
    }
}
