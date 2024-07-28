use crate::{
    infrastructure::{
        db::connection::DBPool,
        repositories::entities::house_second_hand::{
            HouseSecondHandListedPO, HouseSecondHandSoldPO, HouseSecondHandUnlistedPO,
        },
    },
    schema::{house_second_hand_listed, house_second_hand_sale, house_second_hand_unlisted},
};
use bigdecimal::BigDecimal;
use diesel::prelude::Insertable;
use diesel::RunQueryDsl;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, Insertable)]
#[diesel(table_name = house_second_hand_listed)]
pub struct NewHouseSecondHandListedDto {
    house_id: String,
    pice: BigDecimal,
    low_pice: Option<BigDecimal>,
}

impl NewHouseSecondHandListedDto {
    pub fn new(house_id: String, pice: BigDecimal, low_pice: Option<BigDecimal>) -> Self {
        Self {
            house_id,
            pice,
            low_pice,
        }
    }
    pub async fn create(&self, pool: DBPool) -> Result<(), diesel::result::Error> {
        let mut conn = pool.get().unwrap();
        diesel::insert_into(house_second_hand_listed::table)
            .values(self)
            .execute(&mut conn)
            .expect("Error saving new house");

        Ok(())
    }
}

pub struct QueryHouseSecondHandListedDto {}

impl QueryHouseSecondHandListedDto {
    pub fn list(&self, pool: DBPool) -> Vec<HouseSecondHandListedPO> {
        use crate::schema::house_second_hand_listed::dsl::house_second_hand_listed;

        let mut conn = pool.get().unwrap();
        house_second_hand_listed
            .load::<HouseSecondHandListedPO>(&mut conn)
            .expect("Error loading houses")
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Insertable)]
#[diesel(table_name = house_second_hand_unlisted)]

pub struct NewHouseSecondHandUnlistedDto {
    house_id: String,
}

impl NewHouseSecondHandUnlistedDto {
    pub fn new(house_id: String) -> Self {
        Self { house_id }
    }
    pub async fn create(&self, pool: DBPool) -> Result<(), diesel::result::Error> {
        let mut conn = pool.get().unwrap();
        diesel::insert_into(house_second_hand_unlisted::table)
            .values(self)
            .execute(&mut conn)
            .expect("Error saving new house");

        Ok(())
    }
}

pub struct QueryHouseSecondHandUnlistedDto {}

impl QueryHouseSecondHandUnlistedDto {
    pub fn list(&self, pool: DBPool) -> Vec<HouseSecondHandUnlistedPO> {
        use crate::schema::house_second_hand_unlisted::dsl::house_second_hand_unlisted;
        let mut conn = pool.get().unwrap();

        house_second_hand_unlisted
            .load::<HouseSecondHandUnlistedPO>(&mut conn)
            .expect("Error loading houses")
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Insertable)]
#[diesel(table_name = house_second_hand_sale)]

pub struct NewHouseSecondHandSoldDto {
    sale_price: BigDecimal,
    house_id: String,
}

impl NewHouseSecondHandSoldDto {
    pub fn new(house_id: String, sale_price: BigDecimal) -> Self {
        Self {
            sale_price,
            house_id,
        }
    }

    pub async fn create(&self, pool: DBPool) -> Result<(), diesel::result::Error> {
        let mut conn = pool.get().unwrap();
        diesel::insert_into(house_second_hand_sale::table)
            .values(self)
            .execute(&mut conn)
            .expect("Error saving new house");

        Ok(())
    }
}

pub struct QueryHouseSecondHandSoldDto {}

impl QueryHouseSecondHandSoldDto {
    pub fn list(&self, pool: DBPool) -> Vec<HouseSecondHandSoldPO> {
        use crate::schema::house_second_hand_sale::dsl::house_second_hand_sale;
        let mut conn = pool.get().unwrap();

        house_second_hand_sale
            .load::<HouseSecondHandSoldPO>(&mut conn)
            .expect("Error loading houses")
    }
}
