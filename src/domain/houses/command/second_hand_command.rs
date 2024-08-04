use bigdecimal::BigDecimal;
use serde::{Deserialize, Serialize};

// 上架命令
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecondHandListedCommand {
    pub house_id: String,
}

// 下架命令
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecondHandUnlistedCommand {
    pub house_id: String,
}

// 卖出成功事件
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecondHandSoldCommand {
    pub house_id: String,
    pub sale_price: BigDecimal,
}
