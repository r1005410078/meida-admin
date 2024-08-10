use crate::{
    infrastructure::{
        db::connection::DBPool,
        repositories::entities::house_second_hand::{HouseSecondHandListed, HouseSecondHandSold},
    },
    schema::{house_second_hand, house_second_hand_sold},
};
use bigdecimal::BigDecimal;
use chrono::NaiveDateTime;
use diesel::{
    dsl::{exists, select},
    prelude::AsChangeset,
    ExpressionMethods, QueryDsl, RunQueryDsl,
};
use diesel::{prelude::Insertable, query_dsl::methods::SelectDsl, SelectableHelper};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, AsChangeset, Insertable)]
#[diesel(table_name = house_second_hand)]
pub struct SaveHouseSecondHandListedDto {
    pub house_id: String,
    pub community_name: String,
    pub pice: Option<BigDecimal>,
    pub low_pice: Option<BigDecimal>,
    pub listed: Option<i8>,
    pub listed_time: Option<NaiveDateTime>,
    pub unlisted_time: Option<NaiveDateTime>,
}

impl SaveHouseSecondHandListedDto {
    pub async fn save(&self, pool: DBPool) -> Result<(), diesel::result::Error> {
        use crate::schema::house_second_hand::dsl::*;
        let mut conn = pool.get().unwrap();

        let existed: bool = select(exists(
            house_second_hand.filter(house_id.eq(&self.house_id)),
        ))
        .get_result(&mut conn)
        .expect("Error checking if house_second_hand exists");

        if !existed {
            diesel::insert_into(house_second_hand)
                .values(self)
                .execute(&mut conn)
                .expect("Error saving new house");
        } else {
            diesel::update(house_second_hand)
                .filter(house_id.eq(&self.house_id))
                .set(self)
                .execute(&mut conn)
                .expect("Error saving new house");
        }

        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Insertable)]
#[diesel(table_name = house_second_hand_sold)]
pub struct NewHouseSecondHandSoldDto {
    pub house_id: String,
    pub community_name: String,
    pub days_to_sell: i32,
    pub sold_price: BigDecimal,
    pub sold_time: Option<NaiveDateTime>,
}

impl NewHouseSecondHandSoldDto {
    pub async fn create(&self, pool: DBPool) -> Result<(), diesel::result::Error> {
        let mut conn = pool.get().unwrap();

        diesel::insert_into(house_second_hand_sold::table)
            .values(self)
            .execute(&mut conn)
            .expect("Error saving new house");

        Ok(())
    }
}

// 登记的二手房
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QueryHouseSecondHandDto {
    listed: Option<i8>,
}

impl QueryHouseSecondHandDto {
    pub fn list(&self, pool: DBPool) -> Vec<HouseSecondHandListed> {
        use crate::schema::house;
        use crate::schema::house_second_hand::dsl::*;
        use crate::schema::residential;
        use diesel::JoinOnDsl;

        let mut conn = pool.get().unwrap();

        let mut result = SelectDsl::select(
            house_second_hand
                .inner_join(house::table.on(house::house_id.eq(house_id)))
                .inner_join(residential::table.on(residential::community_name.eq(community_name))),
            HouseSecondHandListed::as_select(),
        )
        .into_boxed();

        if let Some(ref _listed) = self.listed {
            result = result.filter(listed.eq(_listed));
        }

        result
            .load::<HouseSecondHandListed>(&mut conn)
            .expect("Error loading houses")
    }
}

// 出售的二手房
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QueryHouseSecondHandSoldDto {}

impl QueryHouseSecondHandSoldDto {
    pub fn list(&self, pool: DBPool) -> Vec<HouseSecondHandSold> {
        use crate::schema::house;
        use crate::schema::house_second_hand_sold::dsl::*;
        use crate::schema::residential;
        use diesel::JoinOnDsl;

        let mut conn = pool.get().unwrap();

        SelectDsl::select(
            house_second_hand_sold
                .inner_join(house::table.on(house::house_id.eq(house_id)))
                .inner_join(residential::table.on(residential::community_name.eq(community_name))),
            HouseSecondHandSold::as_select(),
        )
        .load::<HouseSecondHandSold>(&mut conn)
        .expect("Error loading houses")
    }
}
