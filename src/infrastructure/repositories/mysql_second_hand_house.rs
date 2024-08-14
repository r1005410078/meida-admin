use super::dao::house_second_hand::{
    NewHouseSecondHandSoldDto, QueryHouseSecondHandDto, QueryHouseSecondHandSoldDto,
    SaveHouseSecondHandListedDto,
};

use super::entities::house_second_hand::{HouseSecondHandListed, HouseSecondHandSold};
use super::mysql_house_repository::MysqlHouseRepository;
use super::object_value::query_value::TableData;
use crate::domain::houses::events::second_hand::{
    SaveSecondHandEvent, SecondHandListedEvent, SecondHandSoldEvent, SecondHandUnlistedEvent,
};

use diesel::{ExpressionMethods, SelectableHelper};
use diesel::{QueryDsl, RunQueryDsl};

//////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
/// 二手房上架下架卖出
//////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
impl MysqlHouseRepository {
    // 保存二手房
    pub async fn save_house_second_hand(
        &self,
        event: SaveSecondHandEvent,
    ) -> Result<(), diesel::result::Error> {
        let dto = SaveHouseSecondHandListedDto {
            house_id: event.house_id,
            community_name: event.community_name,
            listed_time: None,
            unlisted_time: None,
            listed: None,
            comment: Some(event.comment),
            tags: Some(event.tags),
            pice: event.pice,
            low_pice: event.low_pice,
        };
        dto.save(self.pool.clone()).await
    }

    // 上架二手房
    pub async fn listed_house_second_hand(
        &self,
        event: SecondHandListedEvent,
    ) -> Result<(), diesel::result::Error> {
        let dto = SaveHouseSecondHandListedDto {
            house_id: event.house_id,
            community_name: event.community_name,
            listed_time: None,
            unlisted_time: None,
            listed: Some(event.listed),
            pice: None,
            low_pice: None,
            comment: None,
            tags: None,
        };

        dto.save(self.pool.clone()).await
    }

    // 下架二手房
    pub async fn unlisted_house_second_hand(
        &self,
        event: SecondHandUnlistedEvent,
    ) -> Result<(), diesel::result::Error> {
        let dto = SaveHouseSecondHandListedDto {
            house_id: event.house_id,
            community_name: event.community_name,
            unlisted_time: Some(event.unlisted_time),
            listed: Some(event.listed),
            listed_time: None,
            pice: None,
            low_pice: None,
            comment: None,
            tags: None,
        };

        dto.save(self.pool.clone()).await
    }

    // 获取上架的数据
    pub async fn house_second_hand_listed_list(
        &self,
        query: QueryHouseSecondHandDto,
    ) -> TableData<HouseSecondHandListed> {
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

    // 删除二手房
    pub async fn delete_house_second_hand_by_house_id(
        &self,
        input_house_id: String,
    ) -> Result<(), diesel::result::Error> {
        use crate::schema::house_second_hand::dsl::*;
        let conn = &mut self.pool.get().unwrap();
        diesel::delete(house_second_hand.filter(house_id.eq(input_house_id))).execute(conn)?;

        Ok(())
    }

    // 卖出的二手房
    pub async fn house_second_hand_sold_list(
        &self,
        query: QueryHouseSecondHandSoldDto,
    ) -> TableData<HouseSecondHandSold> {
        query.list(self.pool.clone())
    }

    // 根据id查找二手房
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
}
