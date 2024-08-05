use crate::{
    common::event_channel::EventSender,
    domain::houses::{
        events::rental_house::RentalHouseUnListedEvent, repositories::house::HouseRepository,
    },
};

use super::rental_house_unlisted_command::RentalHouseUnListedCommand;

pub struct RentalHouseUnListedCommandHandler<R: HouseRepository> {
    repo: R,
    sender: EventSender<RentalHouseUnListedEvent>,
}

impl<R: HouseRepository> RentalHouseUnListedCommandHandler<R> {
    pub fn new(repo: R, sender: EventSender<RentalHouseUnListedEvent>) -> Self {
        Self { repo, sender }
    }
}

impl<R: HouseRepository> RentalHouseUnListedCommandHandler<R> {
    pub async fn handle(
        &self,
        command: RentalHouseUnListedCommand,
    ) -> Result<(), diesel::result::Error> {
        if let Some(mut aggregate) = self.repo.get_by_id(command.house_id.clone()).await {
            aggregate
                .rental_house_unlisted(command, self.sender.clone())
                .await;
            self.repo.save(&aggregate).await?;
        }

        Ok(())
    }
}
