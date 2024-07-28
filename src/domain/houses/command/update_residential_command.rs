use diesel::prelude::Insertable;
use serde::{Deserialize, Serialize};

use crate::{domain::houses::events::residential::UpdateResidentialEvent, schema::residential};

#[derive(Debug, Clone, Deserialize, Serialize, Insertable)]
#[diesel(table_name = residential)]
pub struct UpdateResidentialCommand {
    pub community_id: String,
    pub name: Option<String>,
    pub address: Option<String>,
    pub city: Option<String>,
    pub state: Option<String>,
    pub postal_code: Option<String>,
    pub year_built: Option<i16>,
    pub community_type: Option<String>,
    pub description: Option<String>,
}

impl From<UpdateResidentialCommand> for UpdateResidentialEvent {
    fn from(value: UpdateResidentialCommand) -> Self {
        Self {
            community_id: value.community_id,
            name: value.name.clone(),
            address: value.address.clone(),
            city: value.city.clone(),
            state: value.state.clone(),
            postal_code: value.postal_code.clone(),
            year_built: value.year_built,
            community_type: value.community_type.clone(),
            description: value.description.clone(),
        }
    }
}
