use std::sync::Arc;

use async_trait::async_trait;

use crate::{
    common::event_channel::EventHandler,
    domain::houses::events::second_hand::{
        SecondHandListedEvent, SecondHandSaleEvent, SecondHandUnlistedEvent,
    },
    infrastructure::repositories::mysql_house_repository::MysqlHouseRepository,
    presentation::service::second_hand::SecondHandService,
};

pub struct SecondHandEventHandler {
    service: SecondHandService,
}

impl SecondHandEventHandler {
    pub fn new(reo: Arc<MysqlHouseRepository>) -> Self {
        Self {
            service: SecondHandService::new(reo),
        }
    }
}

#[async_trait]
impl EventHandler<SecondHandListedEvent> for SecondHandEventHandler {
    async fn handle(&self, event: SecondHandListedEvent) -> anyhow::Result<()> {
        self.service.listed(event).await?;
        println!("创建 SecondHandListedEvent:");

        Ok(())
    }
}

#[async_trait]
impl EventHandler<SecondHandUnlistedEvent> for SecondHandEventHandler {
    async fn handle(&self, event: SecondHandUnlistedEvent) -> anyhow::Result<()> {
        self.service.unlisted(event).await?;
        println!("创建 SecondHandUnlistedEvent:");

        Ok(())
    }
}

#[async_trait]
impl EventHandler<SecondHandSaleEvent> for SecondHandEventHandler {
    async fn handle(&self, event: SecondHandSaleEvent) -> anyhow::Result<()> {
        self.service.sale(event).await?;
        println!("创建 SecondHandSaleEvent:");

        Ok(())
    }
}
