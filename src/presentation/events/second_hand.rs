use std::sync::Arc;

use async_trait::async_trait;

use crate::{
    common::event_channel::EventHandler,
    domain::houses::events::second_hand::{
        NewSecondHandEvent, SecondHandListedEvent, SecondHandSoldEvent, SecondHandUnlistedEvent,
        UpdateSecondHandEvent,
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

// 创建二手房
#[async_trait]
impl EventHandler<NewSecondHandEvent> for SecondHandEventHandler {
    async fn handle(&self, event: NewSecondHandEvent) -> anyhow::Result<()> {
        self.service.create(event).await?;

        Ok(())
    }
}

// 创建二手房
#[async_trait]
impl EventHandler<UpdateSecondHandEvent> for SecondHandEventHandler {
    async fn handle(&self, event: UpdateSecondHandEvent) -> anyhow::Result<()> {
        self.service.update(event).await?;

        Ok(())
    }
}

// 上架二手房
#[async_trait]
impl EventHandler<SecondHandListedEvent> for SecondHandEventHandler {
    async fn handle(&self, event: SecondHandListedEvent) -> anyhow::Result<()> {
        self.service.listed(event).await?;

        Ok(())
    }
}

// 下架二手房
#[async_trait]
impl EventHandler<SecondHandUnlistedEvent> for SecondHandEventHandler {
    async fn handle(&self, event: SecondHandUnlistedEvent) -> anyhow::Result<()> {
        self.service.unlisted(event).await?;
        Ok(())
    }
}

// 二手房卖出
#[async_trait]
impl EventHandler<SecondHandSoldEvent> for SecondHandEventHandler {
    async fn handle(&self, event: SecondHandSoldEvent) -> anyhow::Result<()> {
        self.service.sold(event.clone()).await?;
        Ok(())
    }
}
