use std::sync::Arc;

use async_trait::async_trait;

use crate::{
    common::event_channel::EventHandler,
    domain::houses::events::house::{NewHouseEvent, UpdateHouseEvent},
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
        println!("生成成功 HouseEventHandler:");

        Ok(())
    }
}

#[async_trait]
impl EventHandler<UpdateHouseEvent> for HouseEventHandler {
    async fn handle(&self, event: UpdateHouseEvent) -> anyhow::Result<()> {
        self.service.update(event).await?;
        println!("生成成功 HouseEventHandler:");

        Ok(())
    }
}
