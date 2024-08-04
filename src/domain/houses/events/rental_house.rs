use bigdecimal::BigDecimal;
use diesel::prelude::{AsChangeset, Insertable};
use serde::{Deserialize, Serialize};

use crate::schema::house_rental;

#[derive(Debug, Insertable, AsChangeset, Serialize, Deserialize)]
#[diesel(table_name = house_rental)]
pub struct SaveRentalHouseEvent {
    pub house_id: String,
    pub community_name: String,
    pub rent_pice: BigDecimal,
    pub rent_low_pice: Option<BigDecimal>,
}
