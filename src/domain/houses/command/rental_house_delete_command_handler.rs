use crate::{
    common::event_channel::EventSender,
    domain::houses::{
        events::rental_house::DeleteRentalHouseEvent, repositories::house::HouseRepository,
    },
};

use super::rental_house_delete_command::DeleteRentalHouseCommand;

pub struct DeleteRentalHouseCommandHandler<R: HouseRepository> {
    repo: R,
    sender: EventSender<DeleteRentalHouseEvent>,
}

impl<R: HouseRepository> DeleteRentalHouseCommandHandler<R> {
    pub fn new(repo: R, sender: EventSender<DeleteRentalHouseEvent>) -> Self {
        Self { repo, sender }
    }
}

impl<R: HouseRepository> DeleteRentalHouseCommandHandler<R> {
    pub async fn handle(
        &self,
        command: DeleteRentalHouseCommand,
    ) -> Result<(), diesel::result::Error> {
        if let Some(mut aggregate) = self.repo.get_by_id(command.house_id.clone()).await {
            aggregate
                .deleted_rental_house(command, self.sender.clone())
                .await;
            self.repo.save(&mut aggregate).await?;
        }

        Ok(())
    }
}
