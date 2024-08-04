use chrono::NaiveDateTime;
use diesel::{prelude::Queryable, Selectable};
use serde::{Deserialize, Serialize};

use crate::schema::residential;

#[derive(Debug, Clone, Serialize, Deserialize, Queryable, Selectable)]
#[diesel(table_name = residential)]
pub struct Residential {
    pub community_name: String,
    pub region: String,
    pub city: String,
    pub state: String,
    pub postal_code: String,
    pub year_built: i16,
    pub community_type: String,
    pub property_management_company: String,
    pub description: Option<String>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}
