use crate::{
    domain::houses::{
        aggregates::residential::ResidentialAggregate,
        entities::residential::Residential,
        events::residential::{NewResidentialEvent, UpdateResidentialEvent},
        repositories::residential::ResidentialRepository,
    },
    infrastructure::db::connection::{establish_connection, DBPool},
    schema::residential,
};
use async_trait::async_trait;
use diesel::OptionalExtension;
use diesel::{query_dsl::QueryDsl, ExpressionMethods, RunQueryDsl};
use std::sync::Arc;

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

    pub async fn create(
        &self,
        input_residential: NewResidentialEvent,
    ) -> Result<(), diesel::result::Error> {
        let mut conn = self.pool.get().unwrap();
        diesel::insert_into(residential::table)
            .values(input_residential)
            .execute(&mut conn)?;

        Ok(())
    }

    pub async fn update(
        &self,
        input_residential: &UpdateResidentialEvent,
    ) -> Result<(), diesel::result::Error> {
        use crate::schema::residential::dsl::*;
        let mut conn = self.pool.get().unwrap();
        diesel::update(residential)
            .filter(community_name.eq(input_residential.community_name.clone()))
            .set(input_residential)
            .execute(&mut conn)?;
        Ok(())
    }

    pub async fn list(&self) -> Vec<Residential> {
        use crate::schema::residential::dsl::*;
        let conn = &mut self.pool.get().unwrap();
        residential
            .get_results::<Residential>(conn)
            .expect("Error loading user")
    }

    pub async fn get_residential_by_community_name(
        &self,
        input_community_name: String,
    ) -> Option<Residential> {
        use crate::schema::residential::dsl::*;
        let conn = &mut self.pool.get().unwrap();
        residential
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
        input_aggregate: &ResidentialAggregate,
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
            diesel::update(
                residential_aggregate
                    .filter(community_name.eq(input_aggregate.community_name.clone())),
            )
            .set(input_aggregate)
            .execute(&mut conn)?;
        } else {
            diesel::insert_into(residential_aggregate)
                .values(input_aggregate)
                .execute(&mut self.pool.get().unwrap())?;
        }

        Ok(())
    }

    async fn get_by_id(&self, input_community_name: &String) -> Option<ResidentialAggregate> {
        use crate::schema::residential_aggregate::dsl::*;
        let mut conn = self.pool.get().unwrap();

        residential_aggregate
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
