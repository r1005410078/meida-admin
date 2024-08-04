use diesel::prelude::{AsChangeset, Insertable, Queryable};
use serde::{Deserialize, Serialize};

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
    pub community_name: String,
    pub region: String, // 小区地址
    pub city: String,   // 城市
    pub state: String,  // 省份
}

impl ResidentialAggregate {
    pub async fn add_residential(
        command: NewResidentialCommand,
        sender: EventSender<NewResidentialEvent>,
    ) -> anyhow::Result<Self> {
        sender
            .send(command.convert_event(command.community_name.clone()))
            .await?;

        Ok(Self {
            community_name: command.community_name,
            region: command.region,
            city: command.city,
            state: command.state.clone(),
        })
    }

    pub async fn update_residential(
        &mut self,
        command: UpdateResidentialCommand,
        sender: EventSender<UpdateResidentialEvent>,
    ) -> anyhow::Result<()> {
        if let Some(ref region) = command.region {
            self.region = region.clone();
        }

        if let Some(ref city) = command.city {
            self.city = city.clone();
        }

        if let Some(ref state) = command.state {
            self.state = state.clone();
        }

        sender.send(command.into()).await?;

        Ok(())
    }
}
