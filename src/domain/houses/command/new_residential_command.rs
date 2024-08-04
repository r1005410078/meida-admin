use diesel::prelude::Insertable;
use serde::{Deserialize, Serialize};

use crate::{domain::houses::events::residential::NewResidentialEvent, schema::residential};

#[derive(Debug, Clone, Deserialize, Serialize, Insertable)]
#[diesel(table_name = residential)]
pub struct NewResidentialCommand {
    pub community_name: String,
    pub region: String,
    pub city: String,
    pub state: String,
    pub postal_code: String,
    pub year_built: i16,
    pub community_type: String,
    pub description: Option<String>,
    pub property_management_company: Option<String>,
}

impl NewResidentialCommand {
    pub fn convert_event(&self, community_name: String) -> NewResidentialEvent {
        NewResidentialEvent {
            community_name: community_name.clone(),
            region: self.region.clone(),
            city: self.city.clone(),
            state: self.state.clone(),
            postal_code: self.postal_code.clone(),
            year_built: self.year_built,
            community_type: self.community_type.clone(),
            description: self.description.clone(),
            property_management_company: self.property_management_company.clone(),
        }
    }
}
