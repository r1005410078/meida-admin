use std::sync::Arc;

use crate::{
    domain::houses::events::rental_house::SaveRentalHouseEvent,
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

    // 获取上架的出租房
    pub async fn list_listed(&self, query: QueryRentalHouseListedDto) -> Vec<RentalHouseListed> {
        self.repo.house_rental_house_listed_list(query).await
    }

    // 获取卖出的出租房
    pub async fn list_sold(&self, query: QueryRentalHouseSoldDto) -> Vec<RentalHouseSold> {
        self.repo.house_rental_house_sold_list(query).await
    }
}
