use bigdecimal::BigDecimal;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NewHouseEvent {
    pub house_id: String,
    pub community_id: String,
    pub house_address: String,
    pub house_type: String,
    pub area: BigDecimal,
    pub bedrooms: i32,
    pub living_rooms: i32,
    pub bathrooms: i32,
    pub orientation: String,
    pub decoration_status: String,
    pub status: String,
    pub house_description: String,
    pub house_image: String,
    pub owner_name: String,
    pub owner_phone: String,
    pub created_by: String,
    pub updated_by: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateHouseEvent {
    pub house_id: String,
    pub community_id: String,
    pub house_address: Option<String>,
    pub house_type: Option<String>,
    pub area: Option<BigDecimal>,
    pub bedrooms: Option<i32>,
    pub living_rooms: Option<i32>,
    pub bathrooms: Option<i32>,
    pub orientation: Option<String>,
    pub decoration_status: Option<String>,
    pub status: Option<String>,
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
