use crate::{
    common::event_channel::EventSender,
    domain::houses::{events::house::DeleteHouseEvent, repositories::house::HouseRepository},
};

use super::delete_house_command::DeleteHouseCommand;

pub struct DeleteHouseCommandHandler<R: HouseRepository> {
    repo: R,
    sender: EventSender<DeleteHouseEvent>,
}

impl<R: HouseRepository> DeleteHouseCommandHandler<R> {
    pub fn new(repo: R, sender: EventSender<DeleteHouseEvent>) -> Self {
        Self { repo, sender }
    }

    pub async fn handle(&self, command: DeleteHouseCommand) -> Result<(), diesel::result::Error> {
        if let Some(mut aggregate) = self.repo.get_by_id(command.house_id.clone()).await {
            aggregate.delete_house(command, self.sender.clone()).await;
            self.repo.save(&aggregate).await?;
        }

        Ok(())
    }
}
