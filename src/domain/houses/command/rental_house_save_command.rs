use bigdecimal::BigDecimal;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct SaveRentalHouseCommand {
    pub house_id: String,
    pub rent_pice: BigDecimal,
    pub rent_low_pice: Option<BigDecimal>,
    pub comment: Option<String>,
    pub tags: Option<String>,

    // 2024-07-27 03:45:54
    pub viewing_method: Option<String>, // '看房方式',       -- 记录看房的方式（如预约、随时可看等）
    pub payment_method: Option<String>, // '付款方式',      -- 记录付款方式（如一次性付款、按揭贷款等）
    pub full_payment_required: Option<bool>, // '是否全款', -- 标识是否必须全款，0 为否，1 为是
    pub urgent_sale: Option<bool>,      // '是否急切',           -- 标识是否急切出售，0 为否，1 为是
}
