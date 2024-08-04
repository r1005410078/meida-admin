use crate::{
    common::event_channel::EventSender,
    domain::houses::{
        events::second_hand::UpdateSecondHandEvent, repositories::house::HouseRepository,
    },
};

use super::second_hand_update_command::UpdateSecondHandCommand;

pub struct UpdateSecondHandCommandHandler<R: HouseRepository> {
    repo: R,
    sender: EventSender<UpdateSecondHandEvent>,
}

impl<R: HouseRepository> UpdateSecondHandCommandHandler<R> {
    pub fn new(repo: R, sender: EventSender<UpdateSecondHandEvent>) -> Self {
        Self { repo, sender }
    }

    pub async fn handle(
        &self,
        command: UpdateSecondHandCommand,
    ) -> Result<(), diesel::result::Error> {
        if let Some(mut aggregate) = self.repo.get_by_id(command.house_id.clone()).await {
            aggregate
                .second_hand_update(command, self.sender.clone())
                .await;

            self.repo.save(&aggregate).await?;
        }

        Ok(())
    }
}
