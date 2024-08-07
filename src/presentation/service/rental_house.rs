use std::sync::Arc;

use crate::{
    domain::houses::events::rental_house::{
        RentalHouseListedEvent, RentalHouseSoldEvent, RentalHouseUnListedEvent,
        SaveRentalHouseEvent,
    },
    infrastructure::repositories::{
        dao::rental_house::{QueryRentalHouseListedDto, QueryRentalHouseSoldDto},
        entities::rental_house::{RentalHouseListed, RentalHouseSold},
        mysql_house_repository::MysqlHouseRepository,
    },
};

pub struct RentalHouseService {
    repo: Arc<MysqlHouseRepository>,
}

impl RentalHouseService {
    pub fn new(repo: Arc<MysqlHouseRepository>) -> Self {
        Self { repo }
    }

    pub async fn save_rental_house(
        &self,
        event: SaveRentalHouseEvent,
    ) -> Result<(), diesel::result::Error> {
        self.repo.save_rental_house(event).await
    }

    // 上架
    pub async fn listed(&self, event: RentalHouseListedEvent) -> Result<(), diesel::result::Error> {
        self.repo.listed_rental_house(event).await
    }

    // 下架
    pub async fn unlisted(
        &self,
        event: RentalHouseUnListedEvent,
    ) -> Result<(), diesel::result::Error> {
        self.repo.unlisted_rental_house(event).await
    }

    // 出租房租出
    pub async fn sold(&self, event: RentalHouseSoldEvent) -> Result<(), diesel::result::Error> {
        self.repo.save_sold_rental_house(event.clone()).await?;
        self.repo
            .delete_rental_house_by_house_id(event.house_id)
            .await
    }

    // 获取上架的出租房
    pub async fn list(&self, query: QueryRentalHouseListedDto) -> Vec<RentalHouseListed> {
        self.repo.house_rental_house_listed_list(query).await
    }

    // 获取卖出的出租房
    pub async fn list_sold(&self, query: QueryRentalHouseSoldDto) -> Vec<RentalHouseSold> {
        self.repo.house_rental_house_sold_list(query).await
    }

    // 获取出租房详情
    pub async fn detail(&self, house_id: String) -> Option<RentalHouseListed> {
        self.repo.house_rental_house_by_house_id(house_id).await
    }
}
