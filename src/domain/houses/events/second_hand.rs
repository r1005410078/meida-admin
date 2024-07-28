use bigdecimal::BigDecimal;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecondHandListedEvent {
    pub house_id: String,
    pub pice: BigDecimal,
    pub low_pice: Option<BigDecimal>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecondHandUnlistedEvent {
    pub house_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecondHandSaleEvent {
    pub house_id: String,
    pub sale_price: BigDecimal,
}
