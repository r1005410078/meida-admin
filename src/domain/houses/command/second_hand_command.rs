use bigdecimal::BigDecimal;
use serde::{Deserialize, Serialize};

use crate::domain::houses::events::second_hand::{
    SecondHandListedEvent, SecondHandSaleEvent, SecondHandUnlistedEvent,
};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecondHandListedCommand {
    pub house_id: String,
    pub pice: BigDecimal,
    pub low_pice: Option<BigDecimal>,
}

impl SecondHandListedCommand {
    pub fn convert_event(&self) -> SecondHandListedEvent {
        SecondHandListedEvent {
            house_id: self.house_id.clone(),
            pice: self.pice.clone(),
            low_pice: self.low_pice.clone(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecondHandUnlistedCommand {
    pub house_id: String,
}

impl SecondHandUnlistedCommand {
    pub fn convert_event(&self) -> SecondHandUnlistedEvent {
        SecondHandUnlistedEvent {
            house_id: self.house_id.clone(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecondHandSaleCommand {
    pub house_id: String,
    pub sale_price: BigDecimal,
}

impl SecondHandSaleCommand {
    pub fn convert_event(&self) -> SecondHandSaleEvent {
        SecondHandSaleEvent {
            sale_price: self.sale_price.clone(),
            house_id: self.house_id.clone(),
        }
    }
}
