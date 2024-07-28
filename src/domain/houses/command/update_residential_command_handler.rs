use std::sync::Arc;

use tokio::sync::mpsc::Sender;

use crate::{
    common::event_channel::EventSender,
    domain::houses::{
        events::residential::UpdateResidentialEvent,
        repositories::residential::ResidentialRepository,
    },
};

use super::update_residential_command::UpdateResidentialCommand;

pub struct UpdateResidentialCommandHandler<R: ResidentialRepository> {
    repository: R,
    sender: EventSender<UpdateResidentialEvent>,
}

impl<R: ResidentialRepository> UpdateResidentialCommandHandler<R> {
    pub fn new(repository: R, sender: Arc<Sender<UpdateResidentialEvent>>) -> Self {
        Self { repository, sender }
    }
}

impl<R: ResidentialRepository> UpdateResidentialCommandHandler<R> {
    pub async fn handle(
        &self,
        command: UpdateResidentialCommand,
    ) -> Result<(), diesel::result::Error> {
        if let Some(mut aggregates) = self.repository.get_by_id(&command.community_id).await {
            aggregates
                .update_residential(command, self.sender.clone())
                .await;

            self.repository.save(&aggregates).await?;
        };

        Ok(())
    }
}
