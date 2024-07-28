use bigdecimal::BigDecimal;
use chrono::NaiveDateTime;
use diesel::prelude::Queryable;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, Queryable)]
pub struct HouseSecondHandListedPO {
    id: i32,
    house_id: String,
    pice: BigDecimal,
    low_pice: Option<BigDecimal>,
    event_time: Option<NaiveDateTime>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Queryable)]
pub struct HouseSecondHandUnlistedPO {
    id: i32,
    house_id: String,
    event_time: Option<NaiveDateTime>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Queryable)]
pub struct HouseSecondHandSoldPO {
    id: i32,
    sale_price: BigDecimal,
    house_id: String,
    event_time: Option<NaiveDateTime>,
}
