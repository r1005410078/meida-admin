use bigdecimal::BigDecimal;
use serde::{Deserialize, Serialize};

// 更新命令
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SaveSecondHandCommand {
    pub house_id: String,
    pub pice: Option<BigDecimal>,
    pub low_pice: Option<BigDecimal>,
    pub comment: Option<String>,
    pub tags: Option<String>,

    // 2024-07-27 03:45:54
    pub down_payment: Option<BigDecimal>, // '首付' 记录首付金额，精度为两位小数
    pub viewing_method: Option<String>,   // '看房方式' 记录看房的方式（如预约、随时可看等）
    pub payment_method: Option<String>,   // '付款方式' 记录付款方式（如一次性付款、按揭贷款等）
    pub taxes_and_fees: Option<BigDecimal>, // '房源税费' 记录房源涉及的税费，精度为两位小数
    pub full_payment_required: Option<bool>, //  '是否全款'  标识是否必须全款，0 为否，1 为是
    pub urgent_sale: Option<bool>,        // '是否急切' 标识是否急切出售，0 为否，1 为是
}
