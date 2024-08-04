use std::sync::Arc;

use crate::{
    domain::houses::events::second_hand::{
        NewSecondHandEvent, SecondHandListedEvent, SecondHandSoldEvent, SecondHandUnlistedEvent,
        UpdateSecondHandEvent,
    },
    infrastructure::repositories::{
        dao::house_second_hand::{QueryHouseSecondHandDto, QueryHouseSecondHandSoldDto},
        entities::house_second_hand::{
            HouseSecondHandListed, HouseSecondHandSold, HouseSecondHandSoldPO,
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

    // 创建二手房
    pub async fn create(&self, event: NewSecondHandEvent) -> Result<(), diesel::result::Error> {
        self.repo.create_house_second_hand(event).await
    }

    // 更新二手房
    pub async fn update(&self, event: UpdateSecondHandEvent) -> Result<(), diesel::result::Error> {
        self.repo.update_house_second_hand(event).await
    }

    // 保存上架数据
    pub async fn listed(&self, event: SecondHandListedEvent) -> Result<(), diesel::result::Error> {
        self.repo.save_house_second_hand(event).await
    }

    // 获取上架的数据
    pub async fn list_listed(&self, query: QueryHouseSecondHandDto) -> Vec<HouseSecondHandListed> {
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
    pub async fn list_sold(&self, query: QueryHouseSecondHandSoldDto) -> Vec<HouseSecondHandSold> {
        self.repo.house_second_hand_sold_list(query).await
    }
}
