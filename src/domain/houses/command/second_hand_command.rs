use bigdecimal::BigDecimal;
use serde::{Deserialize, Serialize};

// 上架命令
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecondHandListedCommand {
    pub house_id: String,
    pub updated_by: Option<String>,
    pub created_by: Option<String>,
}

// 删除销售房
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeleteSecondHandCommand {
    pub house_id: String,
    pub updated_by: Option<String>,
}

// 下架命令
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecondHandUnlistedCommand {
    pub house_id: String,
    pub updated_by: Option<String>,
    pub created_by: Option<String>,
}

// 卖出成功事件
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecondHandSoldCommand {
    pub house_id: String,
    pub sale_price: BigDecimal,
    pub updated_by: Option<String>,
    pub created_by: Option<String>,
}
