use serde::{Deserialize, Serialize};

// 上架/下架出租房
#[derive(Debug, Serialize, Deserialize)]
pub struct RentalHouseListedCommand {
    pub house_id: String,
}
