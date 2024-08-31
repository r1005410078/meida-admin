use crate::{
    common::event_channel::EventSender,
    domain::houses::{
        events::second_hand::SaveSecondHandEvent, repositories::house::HouseRepository,
    },
};

use super::second_hand_save_command::SaveSecondHandCommand;

pub struct SaveSecondHandCommandHandler<R: HouseRepository> {
    repo: R,
    sender: EventSender<SaveSecondHandEvent>,
}

impl<R: HouseRepository> SaveSecondHandCommandHandler<R> {
    pub fn new(repo: R, sender: EventSender<SaveSecondHandEvent>) -> Self {
        Self { repo, sender }
    }

    pub async fn handle(
        &self,
        command: SaveSecondHandCommand,
    ) -> Result<(), diesel::result::Error> {
        if let Some(mut aggregate) = self.repo.get_by_id(command.house_id.clone()).await {
            aggregate
                .second_hand_update(command, self.sender.clone())
                .await;

            self.repo.save(&mut aggregate).await?;
        }

        Ok(())
    }
}
