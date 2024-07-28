use crate::{
    common::event_channel::EventSender,
    domain::houses::{
        aggregates::house::HouseAggregate, events::house::NewHouseEvent,
        repositories::house::HouseRepository,
    },
};

use super::new_house_command::NewHouseCommand;

pub struct NewHouseCommandHandler<R: HouseRepository> {
    repo: R,
    sender: EventSender<NewHouseEvent>,
}

impl<R: HouseRepository> NewHouseCommandHandler<R> {
    pub fn new(repo: R, sender: EventSender<NewHouseEvent>) -> Self {
        Self { repo, sender }
    }

    pub async fn handle(&self, command: NewHouseCommand) -> Result<(), diesel::result::Error> {
        let agg = HouseAggregate::add_house(command, self.sender.clone()).await;
        self.repo.save(&agg).await?;
        Ok(())
    }
}
