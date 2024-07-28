use bigdecimal::BigDecimal;
use chrono::NaiveDateTime;
use diesel::Queryable;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, Queryable)]
pub struct SecondHand {
    house_id: String,
    pice: BigDecimal,
    low_pice: Option<BigDecimal>,
    sale_price: Option<BigDecimal>,
    created_at: Option<NaiveDateTime>,
    updated_at: Option<NaiveDateTime>,
}
