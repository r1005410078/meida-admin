use crate::{
    common::event_channel::EventSender,
    domain::houses::{
        events::rental_house::RentalHouseSoldEvent, repositories::house::HouseRepository,
    },
};

use super::rental_house_sold_command::RentalHouseSoldCommand;

#[derive(Debug, Clone)]
pub struct RentalHouseSoldCommandHandler<R: HouseRepository> {
    repo: R,
    sender: EventSender<RentalHouseSoldEvent>,
}

impl<R: HouseRepository> RentalHouseSoldCommandHandler<R> {
    pub fn new(repo: R, sender: EventSender<RentalHouseSoldEvent>) -> Self {
        Self { repo, sender }
    }
}

impl<R: HouseRepository> RentalHouseSoldCommandHandler<R> {
    pub async fn handle(
        &self,
        command: RentalHouseSoldCommand,
    ) -> Result<(), diesel::result::Error> {
        if let Some(mut aggregate) = self.repo.get_by_id(command.house_id.clone()).await {
            aggregate
                .rental_house_sold(command, self.sender.clone())
                .await;
        }
        Ok(())
    }
}
