use diesel::prelude::Insertable;
use serde::{Deserialize, Serialize};

use crate::{domain::houses::events::residential::UpdateResidentialEvent, schema::residential};

#[derive(Debug, Clone, Deserialize, Serialize, Insertable)]
#[diesel(table_name = residential)]
pub struct UpdateResidentialCommand {
    pub community_name: String,
    pub region: Option<String>,
    pub city: Option<String>,
    pub state: Option<String>,
    pub postal_code: Option<String>,
    pub year_built: Option<i16>,
    pub community_type: Option<String>,
    pub description: Option<String>,
    pub property_management_company: Option<String>,
}

impl From<UpdateResidentialCommand> for UpdateResidentialEvent {
    fn from(value: UpdateResidentialCommand) -> Self {
        Self {
            community_name: value.community_name,
            region: value.region.clone(),
            city: value.city.clone(),
            state: value.state.clone(),
            postal_code: value.postal_code.clone(),
            year_built: value.year_built,
            community_type: value.community_type.clone(),
            property_management_company: value.property_management_company.clone(),
            description: value.description.clone(),
        }
    }
}
