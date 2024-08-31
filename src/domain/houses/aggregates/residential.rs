use chrono::NaiveDateTime;
use diesel::{
    prelude::{AsChangeset, Insertable, Queryable},
    Selectable,
};
use serde::{Deserialize, Serialize};

use crate::{
    common::event_channel::EventSender,
    domain::houses::{
        command::community_save_command::CommunitySaveCommand,
        events::residential::SaveCommunityEvent,
    },
    schema::residential_aggregate,
};

#[derive(Debug, Clone, Serialize, Deserialize, Insertable, Queryable, AsChangeset, Selectable)]
#[diesel(table_name = residential_aggregate)]
pub struct ResidentialAggregate {
    pub community_name: String,
    pub region: String, // 小区地址
    pub city: String,   // 城市
    pub state: String,  // 省份
    // 创建人
    pub created_by: Option<String>,
    // 更新人
    pub updated_by: Option<String>,
    pub created_at: Option<NaiveDateTime>,
    pub updated_at: Option<NaiveDateTime>,
}

impl ResidentialAggregate {
    pub async fn new(
        command: CommunitySaveCommand,
        sender: EventSender<SaveCommunityEvent>,
    ) -> anyhow::Result<Self> {
        let new_command = command.clone();
        let community = Self {
            community_name: new_command.community_name,
            region: new_command.region.unwrap(),
            city: new_command.city.unwrap_or("安庆".to_string()),
            state: new_command.state.unwrap_or("安徽".to_string()),
            created_by: new_command.created_by,
            updated_by: new_command.updated_by,
            created_at: None,
            updated_at: None,
        };

        sender.send(command.into()).await?;

        Ok(community)
    }

    // 保存二手房
    pub async fn save_residential(
        &mut self,
        command: CommunitySaveCommand,
        sender: EventSender<SaveCommunityEvent>,
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
