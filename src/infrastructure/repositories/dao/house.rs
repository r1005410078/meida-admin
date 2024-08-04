use bigdecimal::BigDecimal;
use diesel::prelude::Insertable;
use diesel::Selectable;
use serde::{Deserialize, Serialize};

use crate::domain::houses::events::house::NewHouseEvent;
use crate::schema::house;

#[derive(Debug, Clone, Serialize, Deserialize, Insertable, Selectable)]
#[diesel(table_name = house)]
pub struct NewHouseDto {
    house_id: String,
    community_name: String,
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

impl From<NewHouseEvent> for NewHouseDto {
    fn from(event: NewHouseEvent) -> Self {
        NewHouseDto {
            house_id: event.house_id,
            community_name: event.community_name,
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
