use serde::{Deserialize, Serialize};

// 上架/下架出租房
#[derive(Debug, Serialize, Deserialize)]
pub struct DeleteRentalHouseCommand {
    pub house_id: String,
    pub updated_by: Option<String>,
}
