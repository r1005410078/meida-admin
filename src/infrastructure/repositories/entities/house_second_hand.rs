use bigdecimal::BigDecimal;
use chrono::NaiveDateTime;
use diesel::{prelude::Queryable, Selectable};
use serde::{Deserialize, Serialize};

use crate::{
    domain::houses::entities::{house::HousePO, residential::Residential},
    schema::{house_second_hand, house_second_hand_sold},
};

#[derive(Debug, Clone, Serialize, Deserialize, Queryable, Selectable)]
#[diesel(table_name = house_second_hand)]
pub struct HouseSecondHandListedPO {
    house_id: String,
    community_name: String,
    pice: BigDecimal,
    low_pice: Option<BigDecimal>,
    listed: i8,
    listed_time: Option<NaiveDateTime>,
    unlisted_time: Option<NaiveDateTime>,
    comment: String,
    tags: String,
    // 新增
    down_payment: Option<BigDecimal>, // '首付' 记录首付金额，精度为两位小数
    viewing_method: Option<String>,   // '看房方式' 记录看房的方式（如预约、随时可看等）
    payment_method: Option<String>,   // '付款方式' 记录付款方式（如一次性付款、按揭贷款等）
    taxes_and_fees: Option<BigDecimal>, // '房源税费' 记录房源涉及的税费，精度为两位小数
    full_payment_required: Option<bool>, //  '是否全款'  标识是否必须全款，0 为否，1 为是
    urgent_sale: Option<bool>,        // '是否急切' 标识是否急切出售，0 为否，1 为是

    created_at: NaiveDateTime,
    updated_at: NaiveDateTime,
}

#[derive(Debug, Clone, Serialize, Deserialize, Queryable, Selectable)]
#[diesel(table_name = house_second_hand_sold)]
pub struct HouseSecondHandSoldPO {
    sold_id: i32,
    house_id: String,
    community_name: String,
    days_to_sell: i32,
    sold_price: BigDecimal,
    sold_time: NaiveDateTime,
    created_at: NaiveDateTime,
    updated_at: NaiveDateTime,
}

#[derive(Debug, Clone, Serialize, Deserialize, Queryable, Selectable)]
pub struct HouseSecondHandListed {
    #[diesel(embed)]
    house: HousePO,
    #[diesel(embed)]
    residential: Residential,
    #[diesel(embed)]
    house_second_hand: HouseSecondHandListedPO,
}

#[derive(Debug, Clone, Serialize, Deserialize, Queryable, Selectable)]
pub struct HouseSecondHandSold {
    #[diesel(embed)]
    house: HousePO,
    #[diesel(embed)]
    residential: Residential,
    #[diesel(embed)]
    house_second_hand: HouseSecondHandSoldPO,
}
