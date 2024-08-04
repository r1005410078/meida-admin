use bigdecimal::BigDecimal;
use chrono::NaiveDateTime;
use diesel::{prelude::Queryable, Selectable};
use serde::{Deserialize, Serialize};

use crate::schema::house;

#[derive(Debug, Clone, Serialize, Deserialize, Queryable, Selectable)]
#[diesel(table_name = house)]
pub struct HousePO {
    pub house_id: String,
    pub community_name: String,
    pub house_address: String,
    pub house_type: String,
    pub area: BigDecimal,
    pub bedrooms: i32,
    pub living_rooms: i32,
    pub bathrooms: i32,
    pub orientation: Option<String>,
    pub decoration_status: Option<String>,
    pub status: Option<String>,
    pub house_description: Option<String>,
    pub house_image: Option<String>,
    pub owner_name: String,
    pub owner_phone: String,
    pub created_by: Option<String>,
    pub updated_by: Option<String>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}
