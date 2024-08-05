use crate::{
    common::event_channel::EventSender,
    domain::houses::{
        events::rental_house::RentalHouseListedEvent, repositories::house::HouseRepository,
    },
};

use super::rental_house_listed_command::RentalHouseListedCommand;

pub struct RentalHouseListedCommandHandler<R: HouseRepository> {
    repo: R,
    sender: EventSender<RentalHouseListedEvent>,
}

impl<R: HouseRepository> RentalHouseListedCommandHandler<R> {
    pub fn new(repo: R, sender: EventSender<RentalHouseListedEvent>) -> Self {
        Self { repo, sender }
    }
}

impl<R: HouseRepository> RentalHouseListedCommandHandler<R> {
    pub async fn handle(
        &self,
        command: RentalHouseListedCommand,
    ) -> Result<(), diesel::result::Error> {
        if let Some(mut aggregate) = self.repo.get_by_id(command.house_id.clone()).await {
            aggregate
                .rental_house_listed(command, self.sender.clone())
                .await;
            self.repo.save(&aggregate).await?;
        }

        Ok(())
    }
}
