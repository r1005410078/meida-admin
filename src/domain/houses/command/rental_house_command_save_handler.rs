use crate::{
    common::event_channel::EventSender,
    domain::houses::{
        events::rental_house::SaveRentalHouseEvent, repositories::house::HouseRepository,
    },
};

use super::rental_house_command_save::SaveRentalHouseCommand;

pub struct RentalHouseCommandSaveHandler<R: HouseRepository> {
    repo: R,
    sender: EventSender<SaveRentalHouseEvent>,
}

impl<R: HouseRepository> RentalHouseCommandSaveHandler<R> {
    pub fn new(repo: R, sender: EventSender<SaveRentalHouseEvent>) -> Self {
        Self { repo, sender }
    }
}

impl<R: HouseRepository> RentalHouseCommandSaveHandler<R> {
    pub async fn handle(
        &self,
        command: SaveRentalHouseCommand,
    ) -> Result<(), diesel::result::Error> {
        if let Some(aggregate) = self.repo.get_by_id(command.house_id.clone()).await {
            aggregate
                .save_rental_house(command, self.sender.clone())
                .await;

            self.repo.save(&aggregate).await?;
        }
        Ok(())
    }
}
