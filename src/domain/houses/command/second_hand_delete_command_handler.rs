use crate::{
    common::event_channel::EventSender,
    domain::houses::{
        events::second_hand::DeleteSecondHandEvent, repositories::house::HouseRepository,
    },
};

use super::second_hand_command::DeleteSecondHandCommand;

pub struct DeleteSecondHandCommandHandler<R: HouseRepository> {
    repo: R,
    sender: EventSender<DeleteSecondHandEvent>,
}

impl<R: HouseRepository> DeleteSecondHandCommandHandler<R> {
    pub fn new(repo: R, sender: EventSender<DeleteSecondHandEvent>) -> Self {
        Self { repo, sender }
    }

    pub async fn handle(
        &self,
        command: DeleteSecondHandCommand,
    ) -> Result<(), diesel::result::Error> {
        if let Some(mut aggregate) = self.repo.get_by_id(command.house_id.clone()).await {
            aggregate
                .deleted_second_hand(command, self.sender.clone())
                .await;

            self.repo.save(&mut aggregate).await?;
        }

        Ok(())
    }
}
