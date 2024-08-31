use diesel::prelude::{AsChangeset, Insertable};
use serde::{Deserialize, Serialize};

use crate::schema::residential;

#[derive(Debug, Clone, Serialize, Deserialize, AsChangeset, Insertable)]
#[diesel(table_name = residential)]
pub struct SaveCommunityEvent {
    pub community_name: String,
    pub region: Option<String>,
    pub city: Option<String>,
    pub state: Option<String>,
    pub postal_code: Option<String>,
    pub year_built: Option<i16>,
    pub community_type: Option<String>,
    pub property_management_company: Option<String>,
    pub description: Option<String>,
    pub created_by: Option<String>,
    pub updated_by: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeleteResidentialEvent {
    pub community_name: String,
}
