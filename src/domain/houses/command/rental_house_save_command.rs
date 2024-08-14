use bigdecimal::BigDecimal;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct SaveRentalHouseCommand {
    pub house_id: String,
    pub rent_pice: BigDecimal,
    pub rent_low_pice: Option<BigDecimal>,
}
