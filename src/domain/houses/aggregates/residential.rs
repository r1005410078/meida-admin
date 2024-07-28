use diesel::prelude::{AsChangeset, Insertable, Queryable};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{
    common::event_channel::EventSender,
    domain::houses::{
        command::{
            new_residential_command::NewResidentialCommand,
            update_residential_command::UpdateResidentialCommand,
        },
        events::residential::{NewResidentialEvent, UpdateResidentialEvent},
    },
    schema::residential_aggregate,
};

#[derive(Debug, Clone, Serialize, Deserialize, Insertable, Queryable, AsChangeset)]
#[diesel(table_name = residential_aggregate)]
pub struct ResidentialAggregate {
    pub community_id: String,
    pub name: String,    // 小区名称
    pub address: String, // 小区地址
    pub city: String,    // 城市
    pub state: String,   // 省份
}

impl ResidentialAggregate {
    pub async fn add_residential(
        command: NewResidentialCommand,
        sender: EventSender<NewResidentialEvent>,
    ) -> Self {
        let community_id = Uuid::new_v4().to_string();

        sender
            .send(command.convert_event(community_id.clone()))
            .await
            .unwrap();

        Self {
            community_id,
            name: command.name,
            address: command.address,
            city: command.city,
            state: command.state.clone(),
        }
    }

    pub async fn update_residential(
        &mut self,
        command: UpdateResidentialCommand,
        sender: EventSender<UpdateResidentialEvent>,
    ) {
        self.name = command.name.clone().unwrap();
        self.address = command.address.clone().unwrap();
        self.city = command.city.clone().unwrap();
        self.state = command.state.clone().unwrap();

        sender.send(command.into()).await.unwrap();
    }
}
