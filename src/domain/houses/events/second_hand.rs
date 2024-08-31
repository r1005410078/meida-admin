use bigdecimal::BigDecimal;
use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};

// 保存二手房
#[derive(Debug, Clone, Serialize)]
pub struct SaveSecondHandEvent {
    pub house_id: String,
    pub community_name: String,
    pub pice: Option<BigDecimal>,
    pub low_pice: Option<BigDecimal>,
    pub comment: String,
    pub tags: String,
    // 2024-07-27 03:45:54
    pub down_payment: Option<BigDecimal>, // '首付' 记录首付金额，精度为两位小数
    pub viewing_method: Option<String>,   // '看房方式' 记录看房的方式（如预约、随时可看等）
    pub payment_method: Option<String>,   // '付款方式' 记录付款方式（如一次性付款、按揭贷款等）
    pub taxes_and_fees: Option<BigDecimal>, // '房源税费' 记录房源涉及的税费，精度为两位小数
    pub full_payment_required: Option<bool>, //  '是否全款'  标识是否必须全款，0 为否，1 为是
    pub urgent_sale: Option<bool>,        // '是否急切' 标识是否急切出售，0 为否，1 为是

    pub created_by: Option<String>,
    pub updated_by: Option<String>,
}

// 删除销售房
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeleteSecondHandEvent {
    pub house_id: String,
}

// 上架成功事件
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecondHandListedEvent {
    pub house_id: String,
    pub community_name: String,
    pub listed: i8,
    pub listed_time: Option<NaiveDateTime>,
    pub created_by: Option<String>,
    pub updated_by: Option<String>,
}

// 下架成功事件
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecondHandUnlistedEvent {
    pub house_id: String,
    pub community_name: String,
    pub listed: i8,
    pub unlisted_time: NaiveDateTime,
    pub created_by: Option<String>,
    pub updated_by: Option<String>,
}

// 卖出成功事件
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecondHandSoldEvent {
    pub house_id: String,
    pub community_name: String,
    pub days_to_sell: i32,
    pub sold_price: BigDecimal,
    pub sold_time: NaiveDateTime,
    pub created_by: Option<String>,
    pub updated_by: Option<String>,
}
