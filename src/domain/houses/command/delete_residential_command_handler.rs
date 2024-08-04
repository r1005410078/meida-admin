use crate::{
    common::event_channel::EventSender,
    domain::houses::{
        events::residential::DeleteResidentialEvent,
        repositories::residential::ResidentialRepository,
    },
};

use super::delete_residential_command::DeleteResidentialCommand;

pub struct DeleteResidentialCommandHandler<R: ResidentialRepository> {
    repo: R,
    sender: EventSender<DeleteResidentialEvent>,
}

impl<R: ResidentialRepository> DeleteResidentialCommandHandler<R> {
    pub fn new(repo: R, sender: EventSender<DeleteResidentialEvent>) -> Self {
        Self { repo, sender }
    }

    pub async fn handle(&self, command: DeleteResidentialCommand) -> anyhow::Result<()> {
        self.repo.delete_by_name(&command.community_name).await?;
        self.sender
            .send(DeleteResidentialEvent {
                community_name: command.community_name,
            })
            .await?;

        Ok(())
    }
}
