use crate::{
    common::event_channel::EventSender,
    domain::houses::{
        events::second_hand::NewSecondHandEvent, repositories::house::HouseRepository,
    },
};

use super::second_hand_new_command::NewSecondHandCommand;

pub struct NewSecondHandCommandHandler<R: HouseRepository> {
    repo: R,
    sender: EventSender<NewSecondHandEvent>,
}

impl<R: HouseRepository> NewSecondHandCommandHandler<R> {
    pub fn new(repo: R, sender: EventSender<NewSecondHandEvent>) -> Self {
        Self { repo, sender }
    }

    pub async fn handle(&self, command: NewSecondHandCommand) -> Result<(), diesel::result::Error> {
        if let Some(mut aggregate) = self.repo.get_by_id(command.house_id.clone()).await {
            aggregate
                .second_hand_new(command, self.sender.clone())
                .await;

            self.repo.save(&aggregate).await?;
        }

        Ok(())
    }
}
