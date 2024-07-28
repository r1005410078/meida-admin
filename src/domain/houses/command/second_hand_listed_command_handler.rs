use crate::{
    common::event_channel::EventSender,
    domain::houses::{
        events::second_hand::SecondHandListedEvent, repositories::house::HouseRepository,
    },
};

use super::second_hand_command::SecondHandListedCommand;

pub struct SecondHandListedCommandHandler<R: HouseRepository> {
    repo: R,
    sender: EventSender<SecondHandListedEvent>,
}

impl<R: HouseRepository> SecondHandListedCommandHandler<R> {
    pub fn new(repo: R, sender: EventSender<SecondHandListedEvent>) -> Self {
        Self { repo, sender }
    }

    pub async fn handle(
        &self,
        command: SecondHandListedCommand,
    ) -> Result<(), diesel::result::Error> {
        if let Some(mut aggregate) = self.repo.get_by_id(command.house_id.clone()).await {
            aggregate
                .second_hand_listed(command, self.sender.clone())
                .await;

            self.repo.save(&aggregate).await?;
        }

        Ok(())
    }
}
