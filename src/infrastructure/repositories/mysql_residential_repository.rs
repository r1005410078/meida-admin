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
            .filter(community_id.eq(input_residential.community_id.clone()))
            .set(input_residential)
            .execute(&mut conn)?;
        Ok(())
    }

    pub async fn list(&self) -> Vec<Residential> {
        use crate::schema::residential::dsl::*;
        let mut conn = self.pool.get().unwrap();
        residential
            .load::<Residential>(&mut conn)
            .expect("Error loading user")
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
            .filter(community_id.eq(input_aggregate.community_id.clone()))
            .count()
            .get_result::<i64>(&mut conn)
            .expect("Error loading houses")
            > 0;

        if exist {
            diesel::update(
                residential_aggregate.filter(community_id.eq(input_aggregate.community_id.clone())),
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

    async fn get_by_id(&self, input_id: &String) -> Option<ResidentialAggregate> {
        use crate::schema::residential_aggregate::dsl::*;
        let mut conn = self.pool.get().unwrap();

        residential_aggregate
            .filter(community_id.eq(input_id))
            .first::<ResidentialAggregate>(&mut conn)
            .optional()
            .expect("Error loading user")
    }
}
