use serde::{Deserialize, Serialize};

// 二手房上架状态
#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub enum SecondHandStatus {
    // 已下架
    Listed,
    // 未上架
    Unlisted,
    // 未知
    Unknown,
}

// 出租房上架状态
#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub enum RentalHouseStatus {
    // 已下架
    Listed,
    // 未上架
    Unlisted,
}
