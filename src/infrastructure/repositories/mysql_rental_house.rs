use super::dao::rental_house::{
    QueryRentalHouseListedDto, QueryRentalHouseSoldDto, RentalHouseSoldDao, SaveRentalHouseDao,
};
use super::entities::rental_house::{RentalHouseListed, RentalHouseSold};
use super::mysql_house_repository::MysqlHouseRepository;
use super::object_value::query_value::TableData;
use crate::domain::houses::events::rental_house::{
    RentalHouseListedEvent, RentalHouseSoldEvent, RentalHouseUnListedEvent, SaveRentalHouseEvent,
};

use diesel::OptionalExtension;
use diesel::{ExpressionMethods, SelectableHelper};
use diesel::{QueryDsl, RunQueryDsl};

//////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
/// 出租房上架下架卖出
//////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
impl MysqlHouseRepository {
    // 保存出租房
    pub async fn save_rental_house(
        &self,
        event: SaveRentalHouseEvent,
    ) -> Result<(), diesel::result::Error> {
        let dto: SaveRentalHouseDao = event.into();
        dto.save(self.pool.clone())
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
    ) -> TableData<RentalHouseListed> {
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
    ) -> TableData<RentalHouseSold> {
        query.list(self.pool.clone())
    }
}
