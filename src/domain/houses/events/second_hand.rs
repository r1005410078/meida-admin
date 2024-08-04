use bigdecimal::BigDecimal;
use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};

// 更新二手房
#[derive(Debug, Clone, Serialize)]
pub struct UpdateSecondHandEvent {
    pub house_id: String,
    pub community_name: String,
    pub pice: Option<BigDecimal>,
    pub low_pice: Option<BigDecimal>,
}

// 新增二手房
#[derive(Debug, Clone, Serialize)]
pub struct NewSecondHandEvent {
    pub house_id: String,
    pub community_name: String,
    pub pice: BigDecimal,
    pub low_pice: Option<BigDecimal>,
}

// 上架成功事件
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecondHandListedEvent {
    pub house_id: String,
    pub community_name: String,
    pub listed: i8,
    pub listed_time: Option<NaiveDateTime>,
}

// 下架成功事件
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecondHandUnlistedEvent {
    pub house_id: String,
    pub community_name: String,
    pub listed: i8,
    pub unlisted_time: NaiveDateTime,
}

// 卖出成功事件
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecondHandSoldEvent {
    pub house_id: String,
    pub community_name: String,
    pub days_to_sell: i32,
    pub sold_price: BigDecimal,
    pub sold_time: NaiveDateTime,
}
