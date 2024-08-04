use crate::{
    common::event_channel::EventSender,
    domain::houses::{
        events::second_hand::SecondHandSoldEvent, repositories::house::HouseRepository,
    },
};

use super::second_hand_command::SecondHandSoldCommand;

pub struct SecondHandSaleCommandHandler<R: HouseRepository> {
    repo: R,
    sender: EventSender<SecondHandSoldEvent>,
}

impl<R: HouseRepository> SecondHandSaleCommandHandler<R> {
    pub fn new(repo: R, sender: EventSender<SecondHandSoldEvent>) -> Self {
        Self { repo, sender }
    }

    pub async fn handle(
        &self,
        command: SecondHandSoldCommand,
    ) -> Result<(), diesel::result::Error> {
        if let Some(mut aggregate) = self.repo.get_by_id(command.house_id.clone()).await {
            aggregate
                .second_hand_sale(command, self.sender.clone())
                .await;

            self.repo.save(&aggregate).await?;
        }

        Ok(())
    }
}
