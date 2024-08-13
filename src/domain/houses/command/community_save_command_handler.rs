use std::sync::Arc;

use tokio::sync::mpsc::Sender;

use crate::{
    common::event_channel::EventSender,
    domain::houses::{
        aggregates::residential::ResidentialAggregate, events::residential::SaveCommunityEvent,
        repositories::residential::ResidentialRepository,
    },
};

use super::community_save_command::CommunitySaveCommand;

pub struct CommunitySaveCommandHandler<R: ResidentialRepository> {
    repository: R,
    sender: EventSender<SaveCommunityEvent>,
}

impl<R: ResidentialRepository> CommunitySaveCommandHandler<R> {
    pub fn new(repository: R, sender: Arc<Sender<SaveCommunityEvent>>) -> Self {
        Self { repository, sender }
    }
}

impl<R: ResidentialRepository> CommunitySaveCommandHandler<R> {
    pub async fn handle(&self, command: CommunitySaveCommand) -> anyhow::Result<()> {
        if let Some(mut aggregates) = self.repository.get_by_id(&command.community_name).await {
            aggregates
                .save_residential(command, self.sender.clone())
                .await?;

            self.repository.save(&aggregates).await?;
        } else {
            let aggregate = ResidentialAggregate::new(command, self.sender.clone()).await?;
            self.repository.save(&aggregate).await?;
        }

        Ok(())
    }
}
