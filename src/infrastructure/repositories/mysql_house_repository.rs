use std::sync::Arc;

use super::dao::house::QueryHouseDao;
use super::object_value::query_value::TableData;
use crate::domain::houses::entities::house::HousePO;
use crate::domain::houses::events::house::SaveHouseEvent;
use crate::{
    domain::houses::{aggregates::house::HouseAggregate, repositories::house::HouseRepository},
    infrastructure::db::connection::{establish_connection, DBPool},
};

use diesel::dsl::exists;
use diesel::OptionalExtension;
use diesel::TextExpressionMethods;
use diesel::{select, ExpressionMethods, SelectableHelper};
use diesel::{QueryDsl, RunQueryDsl};

pub struct MysqlHouseRepository {
    pub pool: DBPool,
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
/// 房屋增删改
//////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
impl MysqlHouseRepository {
    pub async fn save_house(&self, input_house: SaveHouseEvent) -> anyhow::Result<()> {
        use crate::schema::house::dsl::*;
        let conn = &mut self.pool.get()?;

        let existed: bool = select(exists(
            house.filter(house_id.eq(input_house.house_id.clone())),
        ))
        .get_result(conn)?;

        if !existed {
            diesel::insert_into(house)
                .values(input_house)
                .execute(conn)?;
        } else {
            diesel::update(house)
                .filter(house_id.eq(input_house.house_id.clone()))
                .set(input_house)
                .execute(conn)?;
        }

        Ok(())
    }

    pub async fn delete(&self, input_house_id: String) -> anyhow::Result<()> {
        use crate::schema::house::dsl::*;
        let mut conn = self.pool.get()?;
        diesel::delete(house.filter(house_id.eq(input_house_id))).execute(&mut conn)?;
        Ok(())
    }

    pub async fn get_by_house_id(&self, input_house_id: String) -> Option<HousePO> {
        use crate::schema::house::dsl::*;
        let mut conn = self.pool.get().unwrap();
        house
            .select(HousePO::as_select())
            .filter(house_id.eq(input_house_id))
            .order_by(updated_at.desc())
            .first::<HousePO>(&mut conn)
            .optional()
            .expect("Error loading house")
    }

    pub async fn list(&self, query: QueryHouseDao) -> TableData<HousePO> {
        query.list(self.pool.clone())
    }

    // 根据户主名称查询
    pub async fn list_by_owner_name(&self, input_owner_name: String) -> Vec<HousePO> {
        use crate::schema::house::dsl::*;
        let mut conn = self.pool.get().unwrap();
        let like_name = format!("%{}%", input_owner_name);

        house
            .select(HousePO::as_select())
            .filter(owner_name.like(like_name))
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
            .filter(house_id.eq(input_agg.house_id.clone()))
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
            .select(HouseAggregate::as_select())
            .filter(house_id.eq(id))
            .first::<HouseAggregate>(&mut conn)
            .optional()
            .expect("Error loading HouseAggregate")
    }
}
