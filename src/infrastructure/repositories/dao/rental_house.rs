use bigdecimal::BigDecimal;
use chrono::NaiveDateTime;
use diesel::SelectableHelper;
use diesel::{
    dsl::exists,
    prelude::{AsChangeset, Insertable},
    query_dsl::methods::SelectDsl,
    select, ExpressionMethods, QueryDsl, RunQueryDsl,
};
use serde::{Deserialize, Serialize};

use crate::domain::houses::events::rental_house::{
    RentalHouseListedEvent, RentalHouseSoldEvent, RentalHouseUnListedEvent,
};
use crate::infrastructure::repositories::entities::rental_house::RentalHouseSold;
use crate::schema::house_rental_sold;
use crate::{
    domain::houses::events::rental_house::SaveRentalHouseEvent,
    infrastructure::{
        db::connection::DBPool, repositories::entities::rental_house::RentalHouseListed,
    },
    schema::house_rental,
};

#[derive(Debug, Insertable, AsChangeset, Serialize, Deserialize)]
#[diesel(table_name = house_rental)]
pub struct SaveRentalHouseDao {
    pub house_id: String,
    pub community_name: Option<String>,
    pub listed: i8,
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
            listed: 0,
            community_name: Some(event.community_name),
            rent_pice: Some(event.rent_pice),
            rent_low_pice: event.rent_low_pice,
        }
    }
}

impl From<RentalHouseListedEvent> for SaveRentalHouseDao {
    fn from(event: RentalHouseListedEvent) -> Self {
        Self {
            house_id: event.house_id,
            listed: event.listed,
            community_name: None,
            rent_pice: None,
            rent_low_pice: None,
        }
    }
}

impl From<RentalHouseUnListedEvent> for SaveRentalHouseDao {
    fn from(event: RentalHouseUnListedEvent) -> Self {
        Self {
            house_id: event.house_id,
            listed: event.listed,
            community_name: None,
            rent_pice: None,
            rent_low_pice: None,
        }
    }
}

// 出租的房源
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QueryRentalHouseListedDto {}

impl QueryRentalHouseListedDto {
    pub fn list(&self, pool: DBPool) -> Vec<RentalHouseListed> {
        use crate::schema::house;
        use crate::schema::house_rental::dsl::*;
        use crate::schema::residential;
        use diesel::JoinOnDsl;

        let mut conn = pool.get().unwrap();

        SelectDsl::select(
            house_rental
                .inner_join(house::table.on(house::house_id.eq(house_id)))
                .inner_join(residential::table.on(residential::community_name.eq(community_name))),
            RentalHouseListed::as_select(),
        )
        .load::<RentalHouseListed>(&mut conn)
        .expect("Error loading houses")
    }
}

// 已出租的房源
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QueryRentalHouseSoldDto {}

impl QueryRentalHouseSoldDto {
    pub fn list(&self, pool: DBPool) -> Vec<RentalHouseSold> {
        use crate::schema::house;
        use crate::schema::house_rental_sold::dsl::*;
        use crate::schema::residential;
        use diesel::JoinOnDsl;

        let mut conn = pool.get().unwrap();

        SelectDsl::select(
            house_rental_sold
                .inner_join(house::table.on(house::house_id.eq(house_id)))
                .inner_join(residential::table.on(residential::community_name.eq(community_name))),
            RentalHouseSold::as_select(),
        )
        .load::<RentalHouseSold>(&mut conn)
        .expect("Error loading houses")
    }
}

#[derive(Debug, Insertable, AsChangeset, Serialize, Deserialize)]
#[diesel(table_name = house_rental_sold)]
pub struct RentalHouseSoldDao {
    pub house_id: String,
    pub community_name: String,
    pub rent_pice: BigDecimal,
    pub rent_start_time: NaiveDateTime,
    pub rent_end_time: NaiveDateTime,
}

impl RentalHouseSoldDao {
    pub fn save(&self, pool: DBPool) -> Result<(), diesel::result::Error> {
        use crate::schema::house_rental_sold::dsl::*;
        let conn = &mut pool.get().unwrap();
        diesel::insert_into(house_rental_sold)
            .values(self)
            .execute(conn)?;
        Ok(())
    }
}

impl From<RentalHouseSoldEvent> for RentalHouseSoldDao {
    fn from(event: RentalHouseSoldEvent) -> Self {
        Self {
            house_id: event.house_id,
            community_name: event.community_name,
            rent_pice: event.rent_pice,
            rent_start_time: event.rent_start_time,
            rent_end_time: event.rent_end_time,
        }
    }
}
