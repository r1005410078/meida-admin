use std::sync::Arc;

use crate::{
    domain::houses::events::second_hand::{
        SecondHandListedEvent, SecondHandSaleEvent, SecondHandUnlistedEvent,
    },
    infrastructure::repositories::{
        dao::house_second_hand::{
            QueryHouseSecondHandListedDto, QueryHouseSecondHandSoldDto,
            QueryHouseSecondHandUnlistedDto,
        },
        entities::house_second_hand::{
            HouseSecondHandListedPO, HouseSecondHandSoldPO, HouseSecondHandUnlistedPO,
        },
        mysql_house_repository::MysqlHouseRepository,
    },
};

pub struct SecondHandService {
    repo: Arc<MysqlHouseRepository>,
}

impl SecondHandService {
    pub fn new(repo: Arc<MysqlHouseRepository>) -> Self {
        Self { repo }
    }

    // 保存上架数据
    pub async fn listed(&self, event: SecondHandListedEvent) -> Result<(), diesel::result::Error> {
        self.repo.create_house_second_hand_listed(event).await
    }

    // 获取上架的数据
    pub async fn list_listed(
        &self,
        query: QueryHouseSecondHandListedDto,
    ) -> Vec<HouseSecondHandListedPO> {
        self.repo.house_second_hand_listed_list(query).await
    }

    // 保存下架数据
    pub async fn unlisted(
        &self,
        event: SecondHandUnlistedEvent,
    ) -> Result<(), diesel::result::Error> {
        self.repo.create_house_second_hand_unlisted(event).await
    }

    // 获取下架的数据
    pub async fn list_unlisted(
        &self,
        query: QueryHouseSecondHandUnlistedDto,
    ) -> Vec<HouseSecondHandUnlistedPO> {
        self.repo.house_second_hand_unlisted_list(query).await
    }

    // 保存卖出数据
    pub async fn sale(&self, event: SecondHandSaleEvent) -> Result<(), diesel::result::Error> {
        self.repo.create_house_second_hand_sold(event).await
    }

    // 获取卖出的数据
    pub async fn list_sold(
        &self,
        query: QueryHouseSecondHandSoldDto,
    ) -> Vec<HouseSecondHandSoldPO> {
        self.repo.house_second_hand_sold_list(query).await
    }
}
