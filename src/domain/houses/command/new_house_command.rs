use bigdecimal::BigDecimal;
use serde::{Deserialize, Serialize};

use crate::domain::houses::events::house::NewHouseEvent;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NewHouseCommand {
    pub community_name: String,
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
}

impl NewHouseCommand {
    pub fn convert_event(&self, house_id: String) -> NewHouseEvent {
        NewHouseEvent {
            house_id: house_id.clone(),
            community_name: self.community_name.clone(),
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
            created_by: "Rong".to_string(),
            updated_by: "Rong".to_string(),
        }
    }
}
