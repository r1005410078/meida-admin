use bigdecimal::BigDecimal;
use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};

use crate::domain::houses::events::house::SaveHouseEvent;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SaveHouseCommand {
    pub house_id: Option<String>,
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
}

impl SaveHouseCommand {
    pub fn convert_event(self, house_id: String) -> SaveHouseEvent {
        SaveHouseEvent {
            house_id,
            community_name: self.community_name,
            house_address: self.house_address,
            floor: self.floor,
            property: self.property,
            house_age: self.house_age,
            area: self.area,
            bedrooms: self.bedrooms,
            living_rooms: self.living_rooms,
            bathrooms: self.bathrooms,
            orientation: self.orientation,
            decoration_status: self.decoration_status,
            house_description: self.house_description,
            house_image: self.house_image,
            owner_name: self.owner_name,
            owner_phone: self.owner_phone,
            updated_by: None,
        }
    }
}
