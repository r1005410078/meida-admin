use std::sync::Arc;

use async_trait::async_trait;

use crate::{
    common::event_channel::EventHandler,
    domain::houses::events::residential::{DeleteResidentialEvent, SaveCommunityEvent},
    infrastructure::repositories::mysql_community_repository::MysqlResidentialRepository,
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
impl EventHandler<SaveCommunityEvent> for ResidentialEventHandler {
    async fn handle(&self, event: SaveCommunityEvent) -> anyhow::Result<()> {
        self.residential_service.save(event).await?;

        Ok(())
    }
}

#[async_trait]
impl EventHandler<DeleteResidentialEvent> for ResidentialEventHandler {
    async fn handle(&self, event: DeleteResidentialEvent) -> anyhow::Result<()> {
        self.residential_service
            .delete_by_name(&event.community_name)
            .await?;

        Ok(())
    }
}
