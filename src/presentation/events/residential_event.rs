use std::sync::Arc;

use async_trait::async_trait;

use crate::{
    common::event_channel::EventHandler,
    domain::houses::events::residential::{NewResidentialEvent, UpdateResidentialEvent},
    infrastructure::repositories::mysql_residential_repository::MysqlResidentialRepository,
    presentation::service::residential::ResidentialService,
};

pub struct ResidentialEventHandler {
    residential_service: ResidentialService,
}

impl ResidentialEventHandler {
    pub fn new(reo: Arc<MysqlResidentialRepository>) -> Self {
        Self {
            residential_service: ResidentialService::new(reo),
        }
    }
}

#[async_trait]
impl EventHandler<NewResidentialEvent> for ResidentialEventHandler {
    async fn handle(&self, event: NewResidentialEvent) -> anyhow::Result<()> {
        self.residential_service.create(event).await?;
        println!("创建 ResidentialEventHandler:");

        Ok(())
    }
}

#[async_trait]
impl EventHandler<UpdateResidentialEvent> for ResidentialEventHandler {
    async fn handle(&self, event: UpdateResidentialEvent) -> anyhow::Result<()> {
        self.residential_service.update(event).await?;
        println!("更新 ResidentialEventHandler:");

        Ok(())
    }
}
