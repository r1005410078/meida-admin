use bigdecimal::BigDecimal;
use serde::{Deserialize, Serialize};

use crate::domain::houses::events::house::UpdateHouseEvent;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateHouseCommand {
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

impl From<UpdateHouseCommand> for UpdateHouseEvent {
    fn from(value: UpdateHouseCommand) -> Self {
        Self {
            house_id: value.house_id,
            community_id: value.community_id,
            house_address: value.house_address,
            house_type: value.house_type,
            area: value.area,
            bedrooms: value.bedrooms,
            living_rooms: value.living_rooms,
            bathrooms: value.bathrooms,
            orientation: value.orientation,
            decoration_status: value.decoration_status,
            status: value.status,
            house_description: value.house_description,
            house_image: value.house_image,
            owner_name: value.owner_name,
            owner_phone: value.owner_phone,
            updated_by: value.updated_by,
        }
    }
}
