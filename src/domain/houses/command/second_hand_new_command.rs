use bigdecimal::BigDecimal;
use serde::{Deserialize, Serialize};

// 新增命令
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NewSecondHandCommand {
    pub house_id: String,
    pub pice: BigDecimal,
    pub low_pice: Option<BigDecimal>,
}
