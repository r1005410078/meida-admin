use std::sync::Arc;

use async_trait::async_trait;

use crate::{
    common::event_channel::EventHandler,
    domain::houses::events::house::{DeleteHouseEvent, SaveHouseEvent},
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
impl EventHandler<SaveHouseEvent> for HouseEventHandler {
    async fn handle(&self, event: SaveHouseEvent) -> anyhow::Result<()> {
        self.service.save(event).await?;
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
