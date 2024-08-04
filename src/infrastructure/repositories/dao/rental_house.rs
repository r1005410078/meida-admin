use std::sync::Arc;

use bigdecimal::BigDecimal;
use diesel::{
    dsl::exists,
    prelude::{AsChangeset, Insertable},
    select, ExpressionMethods, QueryDsl, RunQueryDsl,
};
use serde::{Deserialize, Serialize};

use crate::{
    domain::houses::events::rental_house::SaveRentalHouseEvent,
    infrastructure::db::connection::DBPool, schema::house_rental,
};

#[derive(Debug, Insertable, AsChangeset, Serialize, Deserialize)]
#[diesel(table_name = house_rental)]
pub struct SaveRentalHouseDao {
    pub house_id: String,
    pub community_name: Option<String>,
    pub rent_pice: Option<BigDecimal>,
    pub rent_low_pice: Option<BigDecimal>,
}

impl SaveRentalHouseDao {
    pub fn save(&self, pool: DBPool) -> Result<(), diesel::result::Error> {
        use crate::schema::house_rental::dsl::*;
        let conn = &mut pool.get().unwrap();

        let sean_exists: bool = select(exists(
            house_rental.filter(house_id.eq(self.house_id.clone())),
        ))
        .get_result(conn)
        .expect("Error loading houses");

        if sean_exists {
            diesel::update(house_rental.filter(house_id.eq(self.house_id.clone())))
                .set(self)
                .execute(conn)?;
        } else {
            diesel::insert_into(house_rental)
                .values(self)
                .execute(conn)?;
        }

        Ok(())
    }
}

impl From<SaveRentalHouseEvent> for SaveRentalHouseDao {
    fn from(event: SaveRentalHouseEvent) -> Self {
        Self {
            house_id: event.house_id,
            community_name: Some(event.community_name),
            rent_pice: Some(event.rent_pice),
            rent_low_pice: event.rent_low_pice,
        }
    }
}
