use bigdecimal::BigDecimal;
use serde::{Deserialize, Serialize};

// 更新命令
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateSecondHandCommand {
    pub house_id: String,
    pub pice: Option<BigDecimal>,
    pub low_pice: Option<BigDecimal>,
}
