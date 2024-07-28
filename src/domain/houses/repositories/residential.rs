use async_trait::async_trait;

use crate::domain::houses::aggregates::residential::ResidentialAggregate;

#[async_trait]
pub trait ResidentialRepository {
    async fn save(&self, agg: &ResidentialAggregate) -> Result<(), diesel::result::Error>;

    async fn get_by_id(&self, id: &String) -> Option<ResidentialAggregate>;
}
