use bigdecimal::BigDecimal;
use chrono::NaiveDateTime;
use diesel::{prelude::Queryable, Selectable};
use serde::{Deserialize, Serialize};

use crate::{
    domain::houses::entities::{house::HousePO, residential::Residential},
    schema::{house_rental, house_rental_sold},
};

#[derive(Debug, Clone, Serialize, Deserialize, Queryable, Selectable)]
#[diesel(table_name = house_rental)]
pub struct RentalHouseListedPO {
    house_id: String,
    community_name: String,
    rent_pice: BigDecimal,
    rent_low_pice: Option<BigDecimal>,
    listed: i8,
    listed_time: Option<NaiveDateTime>,
    unlisted_time: Option<NaiveDateTime>,
    comment: Option<String>,
    tags: Option<String>,
    // 2024-07-27 03:45:54
    viewing_method: Option<String>, // '看房方式',       -- 记录看房的方式（如预约、随时可看等）
    payment_method: Option<String>, // '付款方式',      -- 记录付款方式（如一次性付款、按揭贷款等）
    full_payment_required: Option<bool>, // '是否全款', -- 标识是否必须全款，0 为否，1 为是
    urgent_sale: Option<bool>,      // '是否急切',           -- 标识是否急切出售，0 为否，1 为是

    created_by: Option<String>,
    updated_by: Option<String>,
    created_at: NaiveDateTime,
    updated_at: NaiveDateTime,
}

#[derive(Debug, Clone, Serialize, Deserialize, Queryable, Selectable)]
#[diesel(table_name = house_rental_sold)]
pub struct RentalHouseSoldPO {
    sold_id: i32,
    house_id: String,
    community_name: String,
    rent_pice: BigDecimal,
    rent_start_time: NaiveDateTime,
    rent_end_time: NaiveDateTime,
    created_by: Option<String>,
    updated_by: Option<String>,
    created_at: NaiveDateTime,
    updated_at: NaiveDateTime,
}

#[derive(Debug, Clone, Serialize, Deserialize, Queryable, Selectable)]
pub struct RentalHouseListed {
    #[diesel(embed)]
    house: HousePO,
    #[diesel(embed)]
    residential: Residential,
    #[diesel(embed)]
    rental_house: RentalHouseListedPO,
}

#[derive(Debug, Clone, Serialize, Deserialize, Queryable, Selectable)]
pub struct RentalHouseSold {
    #[diesel(embed)]
    house: HousePO,
    #[diesel(embed)]
    residential: Residential,
    #[diesel(embed)]
    rental_house: RentalHouseSoldPO,
}
