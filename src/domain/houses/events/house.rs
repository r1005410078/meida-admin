use bigdecimal::BigDecimal;
use chrono::NaiveDateTime;
use diesel::prelude::{AsChangeset, Insertable};
use serde::{Deserialize, Serialize};

use crate::schema::house;

#[derive(Debug, Clone, Serialize, Deserialize, Insertable, AsChangeset)]
#[diesel(table_name = house)]
pub struct SaveHouseEvent {
    pub house_id: String,
    pub community_name: String,
    pub house_address: Option<String>,
    pub floor: Option<i32>,
    pub property: Option<String>,
    pub house_age: Option<NaiveDateTime>,
    pub area: Option<BigDecimal>,
    pub bedrooms: Option<i32>,
    pub living_rooms: Option<i32>,
    pub bathrooms: Option<i32>,
    pub orientation: Option<String>,
    pub decoration_status: Option<String>,
    pub house_description: Option<String>,
    pub house_image: Option<String>,
    pub owner_name: Option<String>,
    pub owner_phone: Option<String>,
    pub updated_by: Option<String>,
}

#[derive(Debug, Clone, Serialize)]
pub struct DeleteHouseEvent {
    pub house_id: String,
    pub deleted_by: String,
}
