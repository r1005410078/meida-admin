use std::sync::Arc;

use crate::{
    domain::houses::events::second_hand::{
        SaveSecondHandEvent, SecondHandListedEvent, SecondHandSoldEvent, SecondHandUnlistedEvent,
    },
    infrastructure::repositories::{
        dao::house_second_hand::{QueryHouseSecondHandDto, QueryHouseSecondHandSoldDto},
        entities::house_second_hand::{HouseSecondHandListed, HouseSecondHandSold},
        mysql_house_repository::MysqlHouseRepository,
        object_value::query_value::TableData,
    },
};

pub struct SecondHandService {
    repo: Arc<MysqlHouseRepository>,
}

impl SecondHandService {
    pub fn new(repo: Arc<MysqlHouseRepository>) -> Self {
        Self { repo }
    }

    // 保存二手房
    pub async fn save(&self, event: SaveSecondHandEvent) -> Result<(), diesel::result::Error> {
        self.repo.save_house_second_hand(event).await
    }

    // 保存上架数据
    pub async fn listed(&self, event: SecondHandListedEvent) -> Result<(), diesel::result::Error> {
        self.repo.listed_house_second_hand(event).await
    }

    // 获取上架的数据
    pub async fn list_listed(
        &self,
        query: QueryHouseSecondHandDto,
    ) -> TableData<HouseSecondHandListed> {
        self.repo.house_second_hand_listed_list(query).await
    }

    // 根据id获取二手房信息
    pub async fn house_second_hand_by_house_id(
        &self,
        input_house_id: String,
    ) -> HouseSecondHandListed {
        self.repo
            .house_second_hand_by_house_id(input_house_id)
            .into()
    }

    // 保存下架数据
    pub async fn unlisted(
        &self,
        event: SecondHandUnlistedEvent,
    ) -> Result<(), diesel::result::Error> {
        self.repo.unlisted_house_second_hand(event).await
    }

    // 卖出二手房
    pub async fn sold(&self, event: SecondHandSoldEvent) -> Result<(), diesel::result::Error> {
        println!("保存卖出二手房 {:?}", event.clone());

        self.repo.save_sold_house_second_hand(event.clone()).await?;
        self.repo
            .delete_house_second_hand_by_house_id(event.house_id)
            .await
    }

    // 获取卖出的数据
    pub async fn list_sold(
        &self,
        query: QueryHouseSecondHandSoldDto,
    ) -> TableData<HouseSecondHandSold> {
        self.repo.house_second_hand_sold_list(query).await
    }
}
