use serde::{Deserialize, Serialize};

use crate::domain::houses::events::house::DeleteHouseEvent;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeleteHouseCommand {
    pub house_id: String,
    pub deleted_by: String,
}

impl DeleteHouseCommand {
    pub fn convert_event(&self) -> DeleteHouseEvent {
        DeleteHouseEvent {
            house_id: self.house_id.clone(),
            deleted_by: self.deleted_by.clone(),
        }
    }
}
