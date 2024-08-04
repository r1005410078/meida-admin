use serde::{Deserialize, Serialize};

use crate::domain::houses::events::house::DeleteHouseEvent;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeleteHouseCommand {
    pub house_id: String,
}

impl DeleteHouseCommand {
    pub fn new(house_id: String) -> Self {
        Self { house_id }
    }
    pub fn convert_event(&self) -> DeleteHouseEvent {
        DeleteHouseEvent {
            house_id: self.house_id.clone(),
            deleted_by: "admin".to_string(),
        }
    }
}
