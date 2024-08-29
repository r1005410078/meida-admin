use bigdecimal::BigDecimal;
use chrono::NaiveDate;
use diesel::prelude::{AsChangeset, Insertable};
use serde::{Deserialize, Serialize};

use crate::schema::house;

#[derive(Debug, Clone, Serialize, Deserialize, Insertable, AsChangeset)]
#[diesel(table_name = house)]
pub struct SaveHouseEvent {
    pub house_id: String,
    pub community_name: String,
    pub house_address: Option<String>,
    pub floor: Option<i32>,
    pub property: Option<String>,
    pub area: Option<BigDecimal>,
    pub bedrooms: Option<i32>,
    pub living_rooms: Option<i32>,
    pub bathrooms: Option<i32>,
    pub orientation: Option<String>,
    pub decoration_status: Option<String>,
    pub house_description: Option<String>,
    pub house_image: Option<String>,
    pub owner_name: Option<String>,
    pub owner_phone: Option<String>,
    pub updated_by: Option<String>,
    // 2024-07-24 00:00:00
    pub title: Option<String>,            // '房源标题',
    pub recommended_tags: Option<String>, // '推荐标签',
    pub floor_range: Option<String>,
    pub elevator: Option<i32>,                  // '梯',
    pub household: Option<i32>,                 // '户',
    pub balcony: Option<i32>,                   // '阳台',
    pub kitchen: Option<i32>,                   // '厨房',
    pub building_structure: Option<String>,     // '建筑结构',
    pub building_year: Option<NaiveDate>,       // '建筑年代',
    pub property_rights: Option<String>,        // '产权性质',
    pub property_duration: Option<i32>,         // '产权年限',
    pub property_date: Option<NaiveDate>,       // '产权日期',
    pub delivery_date: Option<NaiveDate>,       // '交房日期',
    pub school_qualification: Option<String>,   // '学位',
    pub household_registration: Option<String>, // '户口',
    pub source: Option<String>,                 // '来源',
    pub unique_house: Option<bool>,             // '唯一住房',
    pub facilities: Option<String>,             // '配套',
    pub usable_area: Option<BigDecimal>,        // '使用面积',
    pub current_status: Option<String>,         // '现状',
    pub house_type: Option<String>,             // '房屋类型',
}

#[derive(Debug, Clone, Serialize)]
pub struct DeleteHouseEvent {
    pub house_id: String,
    pub deleted_by: String,
}
