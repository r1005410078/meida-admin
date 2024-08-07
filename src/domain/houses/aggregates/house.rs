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
            rental_house_command_save::SaveRentalHouseCommand,
            rental_house_listed_command::RentalHouseListedCommand,
            rental_house_sold_command::RentalHouseSoldCommand,
            rental_house_unlisted_command::RentalHouseUnListedCommand,
            second_hand_command::{
                SecondHandListedCommand, SecondHandSoldCommand, SecondHandUnlistedCommand,
            },
            second_hand_new_command::NewSecondHandCommand,
            second_hand_update_command::UpdateSecondHandCommand,
            update_house_command::UpdateHouseCommand,
        },
        events::{
            house::{DeleteHouseEvent, NewHouseEvent, UpdateHouseEvent},
            rental_house::{
                RentalHouseListedEvent, RentalHouseSoldEvent, RentalHouseUnListedEvent,
                SaveRentalHouseEvent,
            },
            second_hand::{
                NewSecondHandEvent, SecondHandListedEvent, SecondHandSoldEvent,
                SecondHandUnlistedEvent, UpdateSecondHandEvent,
            },
        },
        object_value::second_hand::{RentalHouseStatus, SecondHandStatus},
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
    pub rental_start_time: Option<NaiveDateTime>,
    // 租房结束时间
    pub rental_end_time: Option<NaiveDateTime>,
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
/// 房源
///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

impl HouseAggregate {
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

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
/// 二手房
///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

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
        if self.second_hand_status() == SecondHandStatus::Listed {
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

    // 二手房状态
    pub fn rental_house_status(&self) -> RentalHouseStatus {
        // 租房已经过期
        let rental_expired = self.rental_end_time > Some(Utc::now().naive_utc());
        // 下架时间大于上架时间
        let is_listed = self.rental_unlisted_time > self.rental_listed_time;
        // 房屋已经被删除
        let is_deleted_house = self.delete_time.is_some();

        if is_deleted_house || is_listed || rental_expired {
            // 下架状态
            return RentalHouseStatus::Unlisted;
        }

        // 是否是下架时间大于上架时间
        let is_unlisted = self.rental_listed_time > self.rental_unlisted_time;
        // 房租没有过期
        let rental_not_expired =
            self.rental_end_time.is_some() && self.rental_end_time < Some(Utc::now().naive_utc());

        // 有上架时间
        if is_unlisted || rental_not_expired {
            // 上架状态
            RentalHouseStatus::Listed
        } else {
            RentalHouseStatus::Unlisted
        }
    }
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
/// 房屋出租
///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

impl HouseAggregate {
    // 保存出租房
    pub async fn save_rental_house(
        &self,
        command: SaveRentalHouseCommand,
        sender: EventSender<SaveRentalHouseEvent>,
    ) {
        sender
            .send(SaveRentalHouseEvent {
                house_id: command.house_id,
                community_name: self.community_name.clone(),
                rent_pice: command.rent_pice,
                rent_low_pice: command.rent_low_pice,
            })
            .await
            .unwrap();
    }

    // 出租房上架
    pub async fn rental_house_listed(
        &mut self,
        command: RentalHouseListedCommand,
        sender: EventSender<RentalHouseListedEvent>,
    ) {
        let status: RentalHouseStatus = self.rental_house_status();
        if status == RentalHouseStatus::Unlisted {
            // 更新上架时间
            self.rental_listed_time = Some(Utc::now().naive_utc());
            sender
                .send(RentalHouseListedEvent {
                    house_id: command.house_id,
                    listed: 1,
                })
                .await
                .unwrap();
        }
    }

    // 出租房下架
    pub async fn rental_house_unlisted(
        &mut self,
        command: RentalHouseUnListedCommand,
        sender: EventSender<RentalHouseUnListedEvent>,
    ) {
        let status: RentalHouseStatus = self.rental_house_status();
        if status == RentalHouseStatus::Listed {
            // 更新下架时间
            self.rental_unlisted_time = Some(Utc::now().naive_utc());
            sender
                .send(RentalHouseUnListedEvent {
                    house_id: command.house_id,
                    listed: 0,
                })
                .await
                .unwrap();
        }
    }

    // 出租房租出
    pub async fn rental_house_sold(
        &mut self,
        command: RentalHouseSoldCommand,
        sender: EventSender<RentalHouseSoldEvent>,
    ) {
        let status: RentalHouseStatus = self.rental_house_status();
        if status == RentalHouseStatus::Listed {
            // 更新卖出时间
            self.rental_end_time = Some(Utc::now().naive_utc());
            sender
                .send(RentalHouseSoldEvent {
                    house_id: command.house_id,
                    community_name: self.community_name.clone(),
                    rent_pice: command.rent_pice,
                    rent_start_time: command.rent_start_time,
                    rent_end_time: command.rent_end_time,
                })
                .await
                .unwrap();
        }
    }
}
