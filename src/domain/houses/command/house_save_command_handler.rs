use crate::{
    common::event_channel::EventSender,
    domain::houses::{
        aggregates::house::HouseAggregate, events::house::SaveHouseEvent,
        repositories::house::HouseRepository,
    },
};

use super::house_save_command::SaveHouseCommand;

pub struct HouseSaveCommandHandler<R: HouseRepository> {
    repo: R,
    sender: EventSender<SaveHouseEvent>,
}

impl<R: HouseRepository> HouseSaveCommandHandler<R> {
    pub fn new(repo: R, sender: EventSender<SaveHouseEvent>) -> Self {
        Self { repo, sender }
    }

    pub async fn handle(&self, command: SaveHouseCommand) -> Result<String, diesel::result::Error> {
        if let Some(ref house_id) = command.house_id {
            if let Some(mut aggregate) = self.repo.get_by_id(house_id.clone()).await {
                aggregate
                    .update_house(command.clone(), self.sender.clone())
                    .await;
                self.repo.save(&aggregate).await?;

                return Ok(house_id.clone());
            }
        }

        let aggregate = HouseAggregate::new(command.clone(), self.sender.clone()).await;
        self.repo.save(&aggregate).await?;
        Ok(aggregate.house_id)
    }
}
