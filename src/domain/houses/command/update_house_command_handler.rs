use crate::{
    common::event_channel::EventSender,
    domain::houses::{events::house::UpdateHouseEvent, repositories::house::HouseRepository},
};

use super::update_house_command::UpdateHouseCommand;

pub struct UpdateHouseCommandHandler<R: HouseRepository> {
    repo: R,
    sender: EventSender<UpdateHouseEvent>,
}

impl<R: HouseRepository> UpdateHouseCommandHandler<R> {
    pub fn new(repo: R, sender: EventSender<UpdateHouseEvent>) -> Self {
        Self { repo, sender }
    }

    pub async fn handle(&self, command: UpdateHouseCommand) -> Result<(), diesel::result::Error> {
        if let Some(mut aggregate) = self.repo.get_by_id(command.house_id.clone()).await {
            aggregate.update_house(command, self.sender.clone()).await;
            self.repo.save(&aggregate).await?;
        }

        Ok(())
    }
}
