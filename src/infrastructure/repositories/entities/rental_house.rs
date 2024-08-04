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
    created_at: NaiveDateTime,
    updated_at: NaiveDateTime,
}

#[derive(Debug, Clone, Serialize, Deserialize, Queryable, Selectable)]
#[diesel(table_name = house_rental_sold)]
pub struct RentalHouseSoldPO {
    house_id: String,
    community_name: String,
    rent_pice: BigDecimal,
    rent_start_time: NaiveDateTime,
    rent_end_time: NaiveDateTime,
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
    house_second_hand: RentalHouseListedPO,
}

#[derive(Debug, Clone, Serialize, Deserialize, Queryable, Selectable)]
pub struct RentalHouseSold {
    #[diesel(embed)]
    house: HousePO,
    #[diesel(embed)]
    residential: Residential,
    #[diesel(embed)]
    house_second_hand: RentalHouseSoldPO,
}
