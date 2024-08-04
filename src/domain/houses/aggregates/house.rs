use chrono::{NaiveDateTime, Utc};
use diesel::{
    prelude::{AsChangeset, Insertable, Queryable},
    Selectable,
};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{
    common::event_channel::EventSender,
    domain::houses::{
        command::{
            delete_house_command::DeleteHouseCommand,
            new_house_command::NewHouseCommand,
            second_hand_command::{
                SecondHandListedCommand, SecondHandSoldCommand, SecondHandUnlistedCommand,
            },
            second_hand_new_command::NewSecondHandCommand,
            second_hand_update_command::UpdateSecondHandCommand,
            update_house_command::UpdateHouseCommand,
        },
        events::{
            house::{DeleteHouseEvent, NewHouseEvent, UpdateHouseEvent},
            second_hand::{
                NewSecondHandEvent, SecondHandListedEvent, SecondHandSoldEvent,
                SecondHandUnlistedEvent, UpdateSecondHandEvent,
            },
        },
        object_value::second_hand::SecondHandStatus,
    },
    schema::house_aggregate,
};

#[derive(
    Debug, Clone, Serialize, Deserialize, Insertable, AsChangeset, Queryable, Default, Selectable,
)]
#[diesel(table_name = house_aggregate)]
pub struct HouseAggregate {
    pub house_id: String,
    pub community_name: String,
    pub house_address: String,
    pub registration_time: Option<NaiveDateTime>,
    // 房屋被删除时间
    pub delete_time: Option<NaiveDateTime>,
    // 卖出时间
    pub second_hand_sale_time: Option<NaiveDateTime>,
    // 二手房上架时间
    pub second_hand_listed_time: Option<NaiveDateTime>,
    // 二手房下架时间
    pub second_hand_unlisted_time: Option<NaiveDateTime>,
    // 租房上架时间
    pub rental_listed_time: Option<NaiveDateTime>,
    // 租房下架时间
    pub rental_unlisted_time: Option<NaiveDateTime>,
    // 租房租出时间
    pub rental_sale_time: Option<NaiveDateTime>,
    // 租房结束时间
    pub rental_end_time: Option<NaiveDateTime>,
}

impl HouseAggregate {
    // 新增二手房
    pub async fn second_hand_new(
        &mut self,
        command: NewSecondHandCommand,
        sender: EventSender<NewSecondHandEvent>,
    ) {
        sender
            .send(NewSecondHandEvent {
                house_id: command.house_id.clone(),
                community_name: self.community_name.clone(),
                pice: command.pice,
                low_pice: command.low_pice,
            })
            .await
            .unwrap();
    }

    // 更新二手房
    pub async fn second_hand_update(
        &mut self,
        command: UpdateSecondHandCommand,
        sender: EventSender<UpdateSecondHandEvent>,
    ) {
        sender
            .send(UpdateSecondHandEvent {
                house_id: command.house_id.clone(),
                community_name: self.community_name.clone(),
                pice: command.pice,
                low_pice: command.low_pice,
            })
            .await
            .unwrap();
    }

    // 二手房上架
    pub async fn second_hand_listed(
        &mut self,
        command: SecondHandListedCommand,
        sender: EventSender<SecondHandListedEvent>,
    ) {
        // 如果给是下架或成来没操作过才能上架
        if vec![SecondHandStatus::Unlisted, SecondHandStatus::Unknown]
            .contains(&self.second_hand_status())
        {
            // 更新上架时间
            self.second_hand_listed_time = Some(Utc::now().naive_utc());

            // 发送事件
            sender
                .send(SecondHandListedEvent {
                    house_id: command.house_id.clone(),
                    community_name: self.community_name.clone(),
                    listed: 1,
                    listed_time: self.second_hand_listed_time,
                })
                .await
                .unwrap();
        }
    }

    // 二手房下架
    pub async fn second_hand_unlisted(
        &mut self,
        command: SecondHandUnlistedCommand,
        sender: EventSender<SecondHandUnlistedEvent>,
    ) {
        // 如果给是上架才能下架
        if self.second_hand_status() == SecondHandStatus::Listed {
            // 更新下架时间
            self.second_hand_unlisted_time = Some(Utc::now().naive_utc());
            // 发送事件
            sender
                .send(SecondHandUnlistedEvent {
                    house_id: command.house_id.clone(),
                    community_name: self.community_name.clone(),
                    listed: 0,
                    unlisted_time: Utc::now().naive_utc(),
                })
                .await
                .unwrap();
        }
    }

    // 二手卖出
    pub async fn second_hand_sale(
        &mut self,
        command: SecondHandSoldCommand,
        sender: EventSender<SecondHandSoldEvent>,
    ) {
        // 如果上架了才能卖出
        if self.second_hand_status() == SecondHandStatus::Listed {}
        // 清除上架时间
        self.second_hand_listed_time.take();
        self.second_hand_unlisted_time.take();
        // 更新卖出时间
        self.second_hand_sale_time = Some(Utc::now().naive_utc());
        // 发送事件
        sender
            .send(SecondHandSoldEvent {
                house_id: command.house_id.clone(),
                community_name: self.community_name.clone(),
                days_to_sell: 0,
                sold_price: command.sale_price.clone(),
                sold_time: self.second_hand_sale_time.unwrap(),
            })
            .await
            .unwrap();
    }

    // 二手房状态
    pub fn second_hand_status(&self) -> SecondHandStatus {
        if self.delete_time.is_some() // 删除时间
            || self.second_hand_unlisted_time > self.second_hand_listed_time
        // 卖出时间
        {
            // 下架状态
            return SecondHandStatus::Unlisted;
        }

        // 有上架时间
        if self.second_hand_listed_time.is_some() {
            // 上架状态
            SecondHandStatus::Listed
        } else {
            SecondHandStatus::Unknown
        }
    }

    // 新建房屋
    pub async fn add_house(command: NewHouseCommand, sender: EventSender<NewHouseEvent>) -> Self {
        let house_id: String = Uuid::new_v4().to_string();

        sender
            .send(command.convert_event(house_id.clone()))
            .await
            .unwrap();

        Self {
            house_id,
            community_name: command.community_name.clone(),
            house_address: command.house_address.clone(),
            ..Default::default()
        }
    }

    // 更新房屋
    pub async fn update_house(
        &mut self,
        command: UpdateHouseCommand,
        sender: EventSender<UpdateHouseEvent>,
    ) {
        self.community_name = command.community_name.clone();
        if let Some(address) = command.house_address.clone() {
            self.house_address = address;
        }

        sender.send(command.clone().into()).await.unwrap();
    }

    // 删除房屋
    pub async fn delete_house(
        &mut self,
        command: DeleteHouseCommand,
        sender: EventSender<DeleteHouseEvent>,
    ) {
        self.delete_time = Some(Utc::now().naive_utc());
        sender.send(command.convert_event()).await.unwrap();
    }
}
