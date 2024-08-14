use bigdecimal::BigDecimal;
use serde::{Deserialize, Serialize};

// 时间区间
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct TimeRange {
    pub start: i64,
    pub end: i64,
}

// 年份区间
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct YearRange {
    pub start: i16,
    pub end: i16,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct BigDecimalRange {
    pub start: Option<BigDecimal>,
    pub end: Option<BigDecimal>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct IntRange {
    pub start: Option<i32>,
    pub end: Option<i32>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct TableData<T> {
    pub data: Vec<T>,
    pub total: i64,
}

impl<T> TableData<T> {
    pub fn new(data: Vec<T>, total: i64) -> Self {
        Self { data, total }
    }
}
