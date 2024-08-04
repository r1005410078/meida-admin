use std::sync::Arc;

use async_trait::async_trait;

use crate::{
    common::event_channel::EventHandler,
    domain::houses::events::house::{DeleteHouseEvent, NewHouseEvent, UpdateHouseEvent},
    infrastructure::repositories::mysql_house_repository::MysqlHouseRepository,
    presentation::service::house::HouseService,
};

pub struct HouseEventHandler {
    service: HouseService,
}

impl HouseEventHandler {
    pub fn new(reo: Arc<MysqlHouseRepository>) -> Self {
        Self {
            service: HouseService::new(reo),
        }
    }
}

#[async_trait]
impl EventHandler<NewHouseEvent> for HouseEventHandler {
    async fn handle(&self, event: NewHouseEvent) -> anyhow::Result<()> {
        self.service.create(event).await?;
        Ok(())
    }
}

#[async_trait]
impl EventHandler<UpdateHouseEvent> for HouseEventHandler {
    async fn handle(&self, event: UpdateHouseEvent) -> anyhow::Result<()> {
        self.service.update(event).await?;
        Ok(())
    }
}

#[async_trait]
impl EventHandler<DeleteHouseEvent> for HouseEventHandler {
    async fn handle(&self, event: DeleteHouseEvent) -> anyhow::Result<()> {
        self.service.delete(event.house_id).await?;
        Ok(())
    }
}
