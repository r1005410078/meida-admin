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
    pub comment: String,
    pub tags: String,
    // 2024-07-27 03:45:54
    pub viewing_method: Option<String>, // '看房方式',       -- 记录看房的方式（如预约、随时可看等）
    pub payment_method: Option<String>, // '付款方式',      -- 记录付款方式（如一次性付款、按揭贷款等）
    pub full_payment_required: Option<bool>, // '是否全款', -- 标识是否必须全款，0 为否，1 为是
    pub urgent_sale: Option<bool>,      // '是否急切',           -- 标识是否急切出售，0 为否，1 为是
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
