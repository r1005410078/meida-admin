use bigdecimal::BigDecimal;
use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RentalHouseSoldCommand {
    pub house_id: String,
    pub rent_pice: BigDecimal,
    pub rent_start_time: NaiveDateTime,
    pub rent_end_time: NaiveDateTime,
}
