use crate::{
    common::event_channel::EventSender,
    domain::houses::{
        events::second_hand::SecondHandUnlistedEvent, repositories::house::HouseRepository,
    },
};

use super::second_hand_command::SecondHandUnlistedCommand;

pub struct SecondHandUnListedCommandHandler<R: HouseRepository> {
    repo: R,
    sender: EventSender<SecondHandUnlistedEvent>,
}

impl<R: HouseRepository> SecondHandUnListedCommandHandler<R> {
    pub fn new(repo: R, sender: EventSender<SecondHandUnlistedEvent>) -> Self {
        Self { repo, sender }
    }

    pub async fn handle(
        &self,
        command: SecondHandUnlistedCommand,
    ) -> Result<(), diesel::result::Error> {
        if let Some(mut aggregate) = self.repo.get_by_id(command.house_id.clone()).await {
            aggregate
                .second_hand_unlisted(command, self.sender.clone())
                .await;

            self.repo.save(&aggregate).await?;
        }

        Ok(())
    }
}
