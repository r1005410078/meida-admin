use std::sync::Arc;

use super::dao::house_second_hand::{
    NewHouseSecondHandListedDto, NewHouseSecondHandSoldDto, QueryHouseSecondHandDto,
    QueryHouseSecondHandSoldDto, UpdateHouseSecondHandListedDto,
};
use super::dao::rental_house::{
    QueryRentalHouseListedDto, QueryRentalHouseSoldDto, RentalHouseSoldDao, SaveRentalHouseDao,
};
use super::entities::house_second_hand::{HouseSecondHandListed, HouseSecondHandSold};
use super::entities::rental_house::{RentalHouseListed, RentalHouseSold};
use crate::domain::houses::entities::house::HousePO;
use crate::domain::houses::events::house::{NewHouseEvent, UpdateHouseEvent};
use crate::domain::houses::events::rental_house::{
    RentalHouseListedEvent, RentalHouseSoldEvent, RentalHouseUnListedEvent, SaveRentalHouseEvent,
};
use crate::domain::houses::events::second_hand::{
    NewSecondHandEvent, SecondHandListedEvent, SecondHandSoldEvent, SecondHandUnlistedEvent,
    UpdateSecondHandEvent,
};
use crate::{
    domain::houses::{aggregates::house::HouseAggregate, repositories::house::HouseRepository},
    infrastructure::db::connection::{establish_connection, DBPool},
    schema::house,
};

use diesel::OptionalExtension;
use diesel::{ExpressionMethods, SelectableHelper};
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
    // 创建二手房
    pub async fn create_house_second_hand(
        &self,
        event: NewSecondHandEvent,
    ) -> Result<(), diesel::result::Error> {
        let dto = NewHouseSecondHandListedDto {
            house_id: event.house_id,
            community_name: event.community_name,
            listed_time: None,
            unlisted_time: None,
            listed: 0,
            pice: event.pice,
            low_pice: event.low_pice,
        };

        dto.insert_into(self.pool.clone()).await
    }

    // 更新二手房
    pub async fn update_house_second_hand(
        &self,
        event: UpdateSecondHandEvent,
    ) -> Result<(), diesel::result::Error> {
        let dto = UpdateHouseSecondHandListedDto {
            house_id: event.house_id,
            community_name: event.community_name,
            listed_time: None,
            unlisted_time: None,
            listed: None,
            pice: event.pice,
            low_pice: event.low_pice,
        };
        dto.update(self.pool.clone()).await
    }

    // 上架二手房
    pub async fn save_house_second_hand(
        &self,
        event: SecondHandListedEvent,
    ) -> Result<(), diesel::result::Error> {
        let dto = UpdateHouseSecondHandListedDto {
            house_id: event.house_id,
            community_name: event.community_name,
            listed_time: event.listed_time,
            unlisted_time: None,
            listed: Some(event.listed),
            pice: None,
            low_pice: None,
        };

        dto.update(self.pool.clone()).await
    }

    // 下架二手房
    pub async fn unlisted_house_second_hand(
        &self,
        event: SecondHandUnlistedEvent,
    ) -> Result<(), diesel::result::Error> {
        let dto = UpdateHouseSecondHandListedDto {
            house_id: event.house_id,
            community_name: event.community_name,
            unlisted_time: Some(event.unlisted_time),
            listed: Some(event.listed),
            listed_time: None,
            pice: None,
            low_pice: None,
        };

        dto.update(self.pool.clone()).await
    }

    // 获取上架的数据
    pub async fn house_second_hand_listed_list(
        &self,
        query: QueryHouseSecondHandDto,
    ) -> Vec<HouseSecondHandListed> {
        query.list(self.pool.clone())
    }

    // 保存卖出二手房
    pub async fn save_sold_house_second_hand(
        &self,
        event: SecondHandSoldEvent,
    ) -> Result<(), diesel::result::Error> {
        let dto: NewHouseSecondHandSoldDto = NewHouseSecondHandSoldDto {
            house_id: event.house_id,
            community_name: event.community_name,
            days_to_sell: event.days_to_sell,
            sold_price: event.sold_price,
            sold_time: Some(event.sold_time),
        };

        dto.create(self.pool.clone()).await
    }

    pub async fn delete_house_second_hand_by_house_id(
        &self,
        input_house_id: String,
    ) -> Result<(), diesel::result::Error> {
        use crate::schema::house_second_hand::dsl::*;
        let conn = &mut self.pool.get().unwrap();
        diesel::delete(house_second_hand.filter(house_id.eq(input_house_id))).execute(conn)?;

        Ok(())
    }

    pub async fn house_second_hand_sold_list(
        &self,
        query: QueryHouseSecondHandSoldDto,
    ) -> Vec<HouseSecondHandSold> {
        query.list(self.pool.clone())
    }

    pub fn house_second_hand_by_house_id(&self, input_house_id: String) -> HouseSecondHandListed {
        use crate::schema::house;
        use crate::schema::house_second_hand::dsl::*;
        use crate::schema::residential;
        use diesel::query_dsl::methods::SelectDsl;
        use diesel::JoinOnDsl;

        let mut conn = self.pool.get().unwrap();

        SelectDsl::select(
            house_second_hand
                .inner_join(house::table.on(house::house_id.eq(house_id)))
                .inner_join(residential::table.on(residential::community_name.eq(community_name)))
                .filter(house_id.eq(input_house_id)),
            HouseSecondHandListed::as_select(),
        )
        .first::<HouseSecondHandListed>(&mut conn)
        .expect("Error loading houses")
    }

    // 保存出租房
    pub async fn save_rental_house(
        &self,
        event: SaveRentalHouseEvent,
    ) -> Result<(), diesel::result::Error> {
        let dot: SaveRentalHouseDao = event.into();
        dot.save(self.pool.clone())
    }

    pub async fn delete_rental_house_by_house_id(
        &self,
        input_house_id: String,
    ) -> Result<(), diesel::result::Error> {
        use crate::schema::house_rental::dsl::*;
        let mut conn = self.pool.get().expect("Error loading houses");
        diesel::delete(house_rental.filter(house_id.eq(input_house_id))).execute(&mut conn)?;
        Ok(())
    }

    // 上架出租房
    pub async fn listed_rental_house(
        &self,
        event: RentalHouseListedEvent,
    ) -> Result<(), diesel::result::Error> {
        let dot: SaveRentalHouseDao = event.into();
        dot.save(self.pool.clone())
    }

    // 下架出租房
    pub async fn unlisted_rental_house(
        &self,
        event: RentalHouseUnListedEvent,
    ) -> Result<(), diesel::result::Error> {
        let dot: SaveRentalHouseDao = event.into();
        dot.save(self.pool.clone())
    }

    // 获取上架的出租房
    pub async fn house_rental_house_listed_list(
        &self,
        query: QueryRentalHouseListedDto,
    ) -> Vec<RentalHouseListed> {
        query.list(self.pool.clone())
    }

    pub async fn house_rental_house_by_house_id(
        &self,
        input_house_id: String,
    ) -> Option<RentalHouseListed> {
        use crate::schema::house;
        use crate::schema::house_rental::dsl::*;
        use crate::schema::residential;
        use diesel::query_dsl::methods::SelectDsl;
        use diesel::JoinOnDsl;

        let mut conn = self.pool.get().unwrap();

        SelectDsl::select(
            house_rental
                .inner_join(house::table.on(house::house_id.eq(house_id)))
                .inner_join(residential::table.on(residential::community_name.eq(community_name)))
                .filter(house_id.eq(input_house_id)),
            RentalHouseListed::as_select(),
        )
        .first::<RentalHouseListed>(&mut conn)
        .optional()
        .expect("Error loading houses")
    }

    // 保存已出租的出租房
    pub async fn save_sold_rental_house(
        &self,
        event: RentalHouseSoldEvent,
    ) -> Result<(), diesel::result::Error> {
        let dot: RentalHouseSoldDao = event.into();
        dot.save(self.pool.clone())
    }

    // 获取已出租的出租房
    pub async fn house_rental_house_sold_list(
        &self,
        query: QueryRentalHouseSoldDto,
    ) -> Vec<RentalHouseSold> {
        query.list(self.pool.clone())
    }
}

//////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
/// 房屋增删改
//////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

impl MysqlHouseRepository {
    pub async fn insert_into(&self, input_house: NewHouseEvent) -> anyhow::Result<()> {
        let mut conn = self.pool.get()?;

        diesel::insert_into(house::table)
            .values(input_house)
            .execute(&mut conn)?;

        Ok(())
    }

    pub async fn update(&self, input_house: UpdateHouseEvent) -> anyhow::Result<()> {
        use crate::schema::house::dsl::*;
        let mut conn = self.pool.get()?;
        diesel::update(house)
            .filter(house_id.eq(input_house.house_id.clone()))
            .set(input_house)
            .execute(&mut conn)?;

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
            .filter(house_id.eq(input_house_id))
            .order_by(updated_at.desc())
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

    // 根据户主名称查询
    pub async fn list_by_owner_name(&self, input_owner_name: String) -> Vec<HousePO> {
        use crate::schema::house::dsl::*;
        let mut conn = self.pool.get().unwrap();
        house
            .filter(owner_name.eq(input_owner_name))
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
