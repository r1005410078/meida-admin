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
            down_payment: event.down_payment, // '首付' 记录首付金额，精度为两位小数
            viewing_method: event.viewing_method, // '看房方式' 记录看房的方式（如预约、随时可看等）
            payment_method: event.payment_method, // '付款方式' 记录付款方式（如一次性付款、按揭贷款等）
            taxes_and_fees: event.taxes_and_fees, // '房源税费' 记录房源涉及的税费，精度为两位小数
            full_payment_required: event.full_payment_required, //  '是否全款'  标识是否必须全款，0 为否，1 为是
            urgent_sale: event.urgent_sale, // '是否急切' 标识是否急切出售，0 为否，1 为是
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
            down_payment: None,          // '首付' 记录首付金额，精度为两位小数
            viewing_method: None,        // '看房方式' 记录看房的方式（如预约、随时可看等）
            payment_method: None,        // '付款方式' 记录付款方式（如一次性付款、按揭贷款等）
            taxes_and_fees: None,        // '房源税费' 记录房源涉及的税费，精度为两位小数
            full_payment_required: None, //  '是否全款'  标识是否必须全款，0 为否，1 为是
            urgent_sale: None,           // '是否急切' 标识是否急切出售，0 为否，1 为是
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
            down_payment: None,
            viewing_method: None,
            payment_method: None,
            taxes_and_fees: None,
            full_payment_required: None,
            urgent_sale: None,
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
