use diesel::prelude::{AsChangeset, Insertable};
use serde::{Deserialize, Serialize};

use crate::schema::residential;

#[derive(Debug, Clone, Serialize, Deserialize, Insertable)]
#[diesel(table_name = residential)]
pub struct NewResidentialEvent {
    pub community_name: String,
    pub region: String,
    pub city: String,
    pub state: String,
    pub postal_code: String,
    pub year_built: i16,
    pub community_type: String,
    pub property_management_company: Option<String>,
    pub description: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, AsChangeset)]
#[diesel(table_name = residential)]
pub struct UpdateResidentialEvent {
    pub community_name: String,
    pub region: Option<String>,
    pub city: Option<String>,
    pub state: Option<String>,
    pub postal_code: Option<String>,
    pub year_built: Option<i16>,
    pub community_type: Option<String>,
    pub property_management_company: Option<String>,
    pub description: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeleteResidentialEvent {
    pub community_name: String,
}
