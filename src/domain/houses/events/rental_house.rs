use bigdecimal::BigDecimal;
use chrono::NaiveDateTime;
use diesel::prelude::{AsChangeset, Insertable};
use serde::{Deserialize, Serialize};

use crate::schema::house_rental;

#[derive(Debug, Insertable, AsChangeset, Serialize, Deserialize, Clone)]
#[diesel(table_name = house_rental)]
pub struct SaveRentalHouseEvent {
    pub house_id: String,
    pub community_name: String,
    pub rent_pice: BigDecimal,
    pub rent_low_pice: Option<BigDecimal>,
}

// 上架出租房
#[derive(Debug, Insertable, AsChangeset, Serialize, Deserialize)]
#[diesel(table_name = house_rental)]
pub struct RentalHouseListedEvent {
    pub house_id: String,
    pub listed: i8,
}

// 下架出租房
#[derive(Debug, Insertable, AsChangeset, Serialize, Deserialize)]
#[diesel(table_name = house_rental)]
pub struct RentalHouseUnListedEvent {
    pub house_id: String,
    pub listed: i8,
}

// 出租房租出
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct RentalHouseSoldEvent {
    pub house_id: String,
    pub community_name: String,
    pub rent_pice: BigDecimal,
    pub rent_start_time: NaiveDateTime,
    pub rent_end_time: NaiveDateTime,
}
