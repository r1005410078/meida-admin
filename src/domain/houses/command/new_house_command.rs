use bigdecimal::BigDecimal;
use serde::{Deserialize, Serialize};

use crate::domain::houses::events::house::NewHouseEvent;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NewHouseCommand {
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

impl NewHouseCommand {
    pub fn convert_event(&self, house_id: String) -> NewHouseEvent {
        NewHouseEvent {
            house_id: house_id.clone(),
            community_id: self.community_id.clone(),
            house_address: self.house_address.clone(),
            house_type: self.house_type.clone(),
            area: self.area.clone(),
            bedrooms: self.bedrooms,
            living_rooms: self.living_rooms,
            bathrooms: self.bathrooms,
            orientation: self.orientation.clone(),
            decoration_status: self.decoration_status.clone(),
            status: self.status.clone(),
            house_description: self.house_description.clone(),
            house_image: self.house_image.clone(),
            owner_name: self.owner_name.clone(),
            owner_phone: self.owner_phone.clone(),
            created_by: self.created_by.clone(),
            updated_by: self.updated_by.clone(),
        }
    }
}
