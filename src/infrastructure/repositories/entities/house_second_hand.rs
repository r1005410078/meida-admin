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
    created_at: NaiveDateTime,
    updated_at: NaiveDateTime,
}

#[derive(Debug, Clone, Serialize, Deserialize, Queryable, Selectable)]
#[diesel(table_name = house_second_hand_sold)]
pub struct HouseSecondHandSoldPO {
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
