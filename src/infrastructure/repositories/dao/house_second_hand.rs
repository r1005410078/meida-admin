use crate::{
    infrastructure::{
        db::connection::DBPool,
        repositories::entities::house_second_hand::{
            HouseSecondHandListed, HouseSecondHandSold, HouseSecondHandSoldPO,
        },
    },
    schema::{house_second_hand, house_second_hand_sold},
};
use bigdecimal::BigDecimal;
use chrono::NaiveDateTime;
use diesel::{prelude::AsChangeset, ExpressionMethods, QueryDsl, RunQueryDsl};
use diesel::{prelude::Insertable, query_dsl::methods::SelectDsl, SelectableHelper};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, Insertable)]
#[diesel(table_name = house_second_hand)]
pub struct NewHouseSecondHandListedDto {
    pub house_id: String,
    pub community_name: String,
    pub pice: BigDecimal,
    pub low_pice: Option<BigDecimal>,
    pub listed: i8,
    pub listed_time: Option<NaiveDateTime>,
    pub unlisted_time: Option<NaiveDateTime>,
}

impl NewHouseSecondHandListedDto {
    pub async fn insert_into(&self, pool: DBPool) -> Result<(), diesel::result::Error> {
        let mut conn: r2d2::PooledConnection<
            diesel::r2d2::ConnectionManager<diesel::MysqlConnection>,
        > = pool.get().unwrap();
        diesel::insert_into(house_second_hand::table)
            .values(self)
            .execute(&mut conn)
            .expect("Error saving new house");

        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, AsChangeset)]
#[diesel(table_name = house_second_hand)]
pub struct UpdateHouseSecondHandListedDto {
    pub house_id: String,
    pub community_name: String,
    pub pice: Option<BigDecimal>,
    pub low_pice: Option<BigDecimal>,
    pub listed: Option<i8>,
    pub listed_time: Option<NaiveDateTime>,
    pub unlisted_time: Option<NaiveDateTime>,
}

impl UpdateHouseSecondHandListedDto {
    pub async fn update(&self, pool: DBPool) -> Result<(), diesel::result::Error> {
        use crate::schema::house_second_hand::dsl::*;
        let mut conn = pool.get().unwrap();
        diesel::update(house_second_hand)
            .filter(house_id.eq(&self.house_id))
            .set(self)
            .execute(&mut conn)
            .expect("Error saving new house");
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
pub struct QueryHouseSecondHandDto {}

impl QueryHouseSecondHandDto {
    pub fn list(&self, pool: DBPool) -> Vec<HouseSecondHandListed> {
        use crate::schema::house;
        use crate::schema::house_second_hand::dsl::*;
        use crate::schema::residential;
        use diesel::JoinOnDsl;

        let mut conn = pool.get().unwrap();

        SelectDsl::select(
            house_second_hand
                .inner_join(house::table.on(house::house_id.eq(house_id)))
                .inner_join(residential::table.on(residential::community_name.eq(community_name))),
            HouseSecondHandListed::as_select(),
        )
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
