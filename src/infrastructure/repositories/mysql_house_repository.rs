use std::sync::Arc;

use super::dao::house::NewHouseDto;
use super::dao::house_second_hand::{
    NewHouseSecondHandListedDto, NewHouseSecondHandSoldDto, NewHouseSecondHandUnlistedDto,
    QueryHouseSecondHandListedDto, QueryHouseSecondHandSoldDto, QueryHouseSecondHandUnlistedDto,
};
use super::entities::house_second_hand::{
    HouseSecondHandListedPO, HouseSecondHandSoldPO, HouseSecondHandUnlistedPO,
};
use crate::domain::houses::entities::house::HousePO;
use crate::domain::houses::events::second_hand::{
    SecondHandListedEvent, SecondHandSaleEvent, SecondHandUnlistedEvent,
};
use crate::{
    domain::houses::{aggregates::house::HouseAggregate, repositories::house::HouseRepository},
    infrastructure::db::connection::{establish_connection, DBPool},
    schema::house,
};
use diesel::OptionalExtension;
use diesel::{BoolExpressionMethods, ExpressionMethods};
use diesel::{QueryDsl, RunQueryDsl};

pub struct MysqlHouseRepository {
    pool: DBPool,
}

impl MysqlHouseRepository {
    pub fn new() -> Self {
        let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
        MysqlHouseRepository {
            pool: establish_connection(&database_url),
        }
    }
}

//////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
/// 二手房上架下架卖出
//////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
impl MysqlHouseRepository {
    pub async fn create_house_second_hand_listed(
        &self,
        event: SecondHandListedEvent,
    ) -> Result<(), diesel::result::Error> {
        NewHouseSecondHandListedDto::new(event.house_id, event.pice, event.low_pice)
            .create(self.pool.clone())
            .await
    }

    pub async fn house_second_hand_listed_list(
        &self,
        query: QueryHouseSecondHandListedDto,
    ) -> Vec<HouseSecondHandListedPO> {
        query.list(self.pool.clone())
    }

    pub async fn create_house_second_hand_unlisted(
        &self,
        event: SecondHandUnlistedEvent,
    ) -> Result<(), diesel::result::Error> {
        NewHouseSecondHandUnlistedDto::new(event.house_id)
            .create(self.pool.clone())
            .await
    }

    pub async fn house_second_hand_unlisted_list(
        &self,
        query: QueryHouseSecondHandUnlistedDto,
    ) -> Vec<HouseSecondHandUnlistedPO> {
        query.list(self.pool.clone())
    }

    pub async fn create_house_second_hand_sold(
        &self,
        event: SecondHandSaleEvent,
    ) -> Result<(), diesel::result::Error> {
        NewHouseSecondHandSoldDto::new(event.house_id, event.sale_price)
            .create(self.pool.clone())
            .await
    }

    pub async fn house_second_hand_sold_list(
        &self,
        query: QueryHouseSecondHandSoldDto,
    ) -> Vec<HouseSecondHandSoldPO> {
        query.list(self.pool.clone())
    }
}

//////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
/// 房屋增删改
//////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

impl MysqlHouseRepository {
    pub async fn generate_house_event_record(
        &self,
        input_house: NewHouseDto,
    ) -> Result<(), diesel::result::Error> {
        let mut conn = self.pool.get().unwrap();

        diesel::insert_into(house::table)
            .values(input_house)
            .execute(&mut conn)
            .expect("Error saving new house");

        Ok(())
    }

    pub async fn get_by_house_id(&self, input_house_id: String) -> Option<HousePO> {
        use crate::schema::house::dsl::*;
        let mut conn = self.pool.get().unwrap();
        house
            .filter(house_id.eq(input_house_id))
            .order_by(event_time.desc())
            .first::<HousePO>(&mut conn)
            .optional()
            .expect("Error loading house")
    }

    pub async fn list(&self) -> Vec<HousePO> {
        use crate::schema::house::dsl::*;

        let mut conn = self.pool.get().unwrap();
        house
            .load::<HousePO>(&mut conn)
            .expect("Error loading houses")
    }
}

//////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
/// 聚合房屋
//////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
impl HouseRepository for Arc<MysqlHouseRepository> {
    async fn save(&self, input_agg: &HouseAggregate) -> Result<(), diesel::result::Error> {
        use crate::schema::house_aggregate::dsl::*;
        let mut conn = self.pool.get().unwrap();
        let exist = house_aggregate
            .filter(
                community_id
                    .eq(input_agg.community_id.clone())
                    .and(house_address.eq(input_agg.house_address.clone())),
            )
            .count()
            .get_result::<i64>(&mut conn)
            .expect("Error loading houses")
            > 0;

        if exist {
            diesel::update(house_aggregate.filter(house_id.eq(input_agg.house_id.clone())))
                .set(input_agg)
                .execute(&mut conn)?;
        } else {
            diesel::insert_into(house_aggregate)
                .values(input_agg)
                .execute(&mut self.pool.get().unwrap())?;
        }

        Ok(())
    }

    async fn get_by_id(&self, id: String) -> Option<HouseAggregate> {
        use crate::schema::house_aggregate::dsl::*;

        let mut conn = self.pool.get().unwrap();
        house_aggregate
            .filter(house_id.eq(id))
            .first::<HouseAggregate>(&mut conn)
            .optional()
            .expect("Error loading HouseAggregate")
    }
}
