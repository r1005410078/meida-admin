use crate::{
    common::event_channel::EventSender,
    domain::houses::{
        aggregates::residential::ResidentialAggregate, events::residential::NewResidentialEvent,
        repositories::residential::ResidentialRepository,
    },
};

use super::new_residential_command::NewResidentialCommand;

pub struct NewResidentialCommandHandler<R: ResidentialRepository> {
    repository: R,
    sender: EventSender<NewResidentialEvent>,
}

impl<R: ResidentialRepository> NewResidentialCommandHandler<R> {
    pub fn new(repository: R, sender: EventSender<NewResidentialEvent>) -> Self {
        Self { repository, sender }
    }
}

impl<R: ResidentialRepository> NewResidentialCommandHandler<R> {
    pub async fn handle(
        &self,
        command: NewResidentialCommand,
    ) -> Result<(), diesel::result::Error> {
        let aggregates = ResidentialAggregate::add_residential(command, self.sender.clone()).await;
        self.repository.save(&aggregates).await?;

        Ok(())
    }
}
