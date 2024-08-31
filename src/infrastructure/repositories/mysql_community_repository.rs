use crate::{
    domain::houses::{
        aggregates::residential::ResidentialAggregate, entities::residential::Residential,
        events::residential::SaveCommunityEvent, repositories::residential::ResidentialRepository,
    },
    infrastructure::db::connection::{establish_connection, DBPool},
};
use async_trait::async_trait;
use diesel::{
    dsl::{exists, select},
    OptionalExtension, SelectableHelper,
};
use diesel::{query_dsl::QueryDsl, ExpressionMethods, RunQueryDsl};
use std::sync::Arc;

use super::dao::community::QueryCommunityDao;

pub struct MysqlResidentialRepository {
    pool: DBPool,
}

impl MysqlResidentialRepository {
    pub fn new() -> Self {
        let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
        MysqlResidentialRepository {
            pool: establish_connection(&database_url),
        }
    }

    pub async fn save_community(
        &self,
        input_residential: &mut SaveCommunityEvent,
    ) -> Result<(), diesel::result::Error> {
        use crate::schema::residential::dsl::*;
        let conn = &mut self.pool.get().unwrap();

        let existed: bool = select(exists(
            residential.filter(community_name.eq(&input_residential.community_name)),
        ))
        .get_result(conn)
        .expect("Error checking if community exists");

        if !existed {
            diesel::insert_into(residential)
                .values(input_residential.clone())
                .execute(conn)?;
        } else {
            input_residential.created_by.take();
            diesel::update(residential)
                .filter(community_name.eq(input_residential.community_name.clone()))
                .set(input_residential.clone())
                .execute(conn)?;
        }

        Ok(())
    }

    pub async fn list(&self, query: QueryCommunityDao) -> Vec<Residential> {
        query.list(self.pool.clone())
    }

    pub async fn get_community_names(&self) -> Vec<String> {
        use crate::schema::residential::dsl::*;
        let conn = &mut self.pool.get().unwrap();
        residential
            .select(community_name)
            .get_results::<String>(conn)
            .expect("Error loading user")
    }

    pub async fn get_residential_by_community_name(
        &self,
        input_community_name: String,
    ) -> Option<Residential> {
        use crate::schema::residential::dsl::*;
        let conn = &mut self.pool.get().unwrap();
        residential
            .select(Residential::as_select())
            .filter(community_name.eq(input_community_name))
            .first::<Residential>(conn)
            .optional()
            .expect("Error loading user")
    }

    pub async fn delete_residential_by_community_name(
        &self,
        input_community_name: &String,
    ) -> Result<(), diesel::result::Error> {
        use crate::schema::residential::dsl::*;
        let mut conn = self.pool.get().unwrap();
        diesel::delete(residential)
            .filter(community_name.eq(input_community_name))
            .execute(&mut conn)?;

        Ok(())
    }
}

#[async_trait]
impl ResidentialRepository for Arc<MysqlResidentialRepository> {
    async fn save(
        &self,
        input_aggregate: &mut ResidentialAggregate,
    ) -> Result<(), diesel::result::Error> {
        use crate::schema::residential_aggregate::dsl::*;

        let mut conn = self.pool.get().unwrap();
        let exist = residential_aggregate
            .filter(community_name.eq(input_aggregate.community_name.clone()))
            .count()
            .get_result::<i64>(&mut conn)
            .expect("Error loading houses")
            > 0;

        if exist {
            input_aggregate.created_by.take();
            diesel::update(
                residential_aggregate
                    .filter(community_name.eq(input_aggregate.community_name.clone())),
            )
            .set(input_aggregate.clone())
            .execute(&mut conn)?;
        } else {
            diesel::insert_into(residential_aggregate)
                .values(input_aggregate.clone())
                .execute(&mut self.pool.get().unwrap())?;
        }

        Ok(())
    }

    async fn get_by_id(&self, input_community_name: &String) -> Option<ResidentialAggregate> {
        use crate::schema::residential_aggregate::dsl::*;
        let mut conn = self.pool.get().unwrap();

        residential_aggregate
            .select(ResidentialAggregate::as_select())
            .filter(community_name.eq(input_community_name))
            .first::<ResidentialAggregate>(&mut conn)
            .optional()
            .expect("Error loading user")
    }

    async fn delete_by_name(
        &self,
        input_community_name: &String,
    ) -> Result<(), diesel::result::Error> {
        use crate::schema::residential_aggregate::dsl::*;
        diesel::delete(residential_aggregate.filter(community_name.eq(input_community_name)))
            .execute(&mut self.pool.get().unwrap())?;
        Ok(())
    }
}
