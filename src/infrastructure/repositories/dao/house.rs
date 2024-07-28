use bigdecimal::BigDecimal;
use chrono::NaiveDateTime;
use diesel::prelude::Insertable;
use diesel::prelude::Queryable;
use diesel::Selectable;
use serde::{Deserialize, Serialize};

use crate::domain::houses::entities::house::HousePO;
use crate::domain::houses::events::house::DeleteHouseEvent;
use crate::domain::houses::events::house::NewHouseEvent;
use crate::domain::houses::events::house::UpdateHouseEvent;
use crate::schema::delete_house;
use crate::schema::house;

#[derive(Debug, Clone, Serialize, Deserialize, Insertable, Selectable)]
#[diesel(table_name = house)]
pub struct NewHouseDto {
    house_id: String,
    community_id: String,
    house_address: String,
    house_type: String,
    area: BigDecimal,
    bedrooms: i32,
    living_rooms: i32,
    bathrooms: i32,
    orientation: String,
    decoration_status: String,
    status: String,
    house_description: String,
    house_image: String,
    owner_name: String,
    owner_phone: String,
    created_by: String,
    updated_by: String,
}

impl NewHouseDto {
    pub fn new_by_event(event: UpdateHouseEvent, house: HousePO) -> Self {
        NewHouseDto {
            house_id: event.house_id,
            community_id: event.community_id,
            house_address: event.house_address.unwrap_or(house.house_address),
            house_type: event.house_type.unwrap_or(house.house_type),
            area: event.area.unwrap_or(house.area),
            bedrooms: event.bedrooms.unwrap_or(house.bedrooms),
            living_rooms: event.living_rooms.unwrap_or(house.living_rooms),
            bathrooms: event.bathrooms.unwrap_or(house.bathrooms),
            orientation: event
                .orientation
                .unwrap_or(house.orientation.unwrap_or_default()),
            decoration_status: event
                .decoration_status
                .unwrap_or(house.decoration_status.unwrap_or_default()),
            status: event.status.unwrap_or(house.status.unwrap_or_default()),
            house_description: event
                .house_description
                .unwrap_or(house.house_description.unwrap_or_default()),
            house_image: event
                .house_image
                .unwrap_or(house.house_image.unwrap_or_default()),
            owner_name: event.owner_name.unwrap_or(house.owner_name),
            owner_phone: event.owner_phone.unwrap_or(house.owner_phone),
            created_by: house.created_by.unwrap_or_default(),
            updated_by: event
                .updated_by
                .unwrap_or(house.updated_by.unwrap_or_default()),
        }
    }
}

impl From<NewHouseEvent> for NewHouseDto {
    fn from(event: NewHouseEvent) -> Self {
        NewHouseDto {
            house_id: event.house_id,
            community_id: event.community_id,
            house_address: event.house_address,
            house_type: event.house_type,
            area: event.area,
            bedrooms: event.bedrooms,
            living_rooms: event.living_rooms,
            bathrooms: event.bathrooms,
            orientation: event.orientation.clone(),
            decoration_status: event.decoration_status.clone(),
            status: event.status.clone(),
            house_description: event.house_description.clone(),
            house_image: event.house_image.clone(),
            owner_name: event.owner_name.clone(),
            owner_phone: event.owner_phone.clone(),
            created_by: event.created_by.clone(),
            updated_by: event.updated_by.clone(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Insertable)]
#[diesel(table_name = delete_house)]
pub struct DeleteHouseDto {
    house_id: String,
    deleted_by: String,
}

impl From<DeleteHouseEvent> for DeleteHouseDto {
    fn from(event: DeleteHouseEvent) -> Self {
        DeleteHouseDto {
            house_id: event.house_id,
            deleted_by: event.deleted_by,
        }
    }
}
