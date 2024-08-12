use crate::{
    domain::houses::entities::residential::Residential,
    infrastructure::{
        db::connection::DBPool,
        repositories::object_value::query_value::{TimeRange, YearRange},
    },
};
use chrono::NaiveDateTime;
use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct QueryCommunityDao {
    pub community_name: Option<String>,
    pub community_type: Option<String>,
    pub region: Option<String>,
    pub year_built: Option<YearRange>,
    pub description: Option<String>,
    pub updated_at: Option<TimeRange>,
}

impl QueryCommunityDao {
    pub fn list(&self, pool: DBPool) -> Vec<Residential> {
        use crate::schema::residential::dsl::*;
        let conn = &mut pool.get().unwrap();
        let mut result = residential.into_boxed();

        if let Some(ref input_community_name) = self.community_name {
            result = result.filter(community_name.eq(input_community_name));
        }

        if let Some(ref input_region) = self.region {
            result = result.filter(region.eq(input_region));
        }

        if let Some(ref input_year_built) = self.year_built {
            result =
                result.filter(year_built.between(input_year_built.start, input_year_built.end));
        }

        if let Some(ref input_description) = self.description {
            result = result.filter(description.eq(input_description));
        }

        if let Some(ref input_community_type) = self.community_type {
            result = result.filter(community_type.eq(input_community_type));
        }

        if let Some(ref input_updated_at) = self.updated_at {
            let start = NaiveDateTime::from_timestamp_nanos(input_updated_at.start).unwrap();
            let end = NaiveDateTime::from_timestamp_nanos(input_updated_at.end).unwrap();
            result = result.filter(updated_at.between(start, end));
        }

        result
            .get_results::<Residential>(conn)
            .expect("Error loading user")
    }
}
