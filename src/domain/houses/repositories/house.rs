use crate::domain::houses::aggregates::house::HouseAggregate;

pub trait HouseRepository {
    async fn save(&self, agg: &mut HouseAggregate) -> Result<(), diesel::result::Error>;
    async fn get_by_id(&self, id: String) -> Option<HouseAggregate>;
}
