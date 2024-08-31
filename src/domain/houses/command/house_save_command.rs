use bigdecimal::BigDecimal;
use chrono::{NaiveDate, NaiveDateTime};
use serde::{Deserialize, Serialize};

use crate::domain::houses::events::house::SaveHouseEvent;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SaveHouseCommand {
    pub house_id: Option<String>,          // '房源id',
    pub community_name: String,            // '小区名',
    pub house_address: Option<String>,     // '房源地址',
    pub floor: Option<i32>,                // '楼层',
    pub property: Option<String>,          // '房屋类型',
    pub house_age: Option<NaiveDateTime>,  // '房龄',
    pub area: Option<BigDecimal>,          // '面积',
    pub bedrooms: Option<i32>,             // '卧室',
    pub living_rooms: Option<i32>,         // '客厅',
    pub bathrooms: Option<i32>,            // '卫生间',
    pub orientation: Option<String>,       // '朝向',
    pub decoration_status: Option<String>, // '装修状况',
    pub house_description: Option<String>, // '房源描述',
    pub house_image: Option<String>,       // '户型图',
    pub owner_name: Option<String>,        // '业主姓名',
    pub owner_phone: Option<String>,       // '业主电话',

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

    pub updated_by: Option<String>,
    pub created_by: Option<String>,
}

impl SaveHouseCommand {
    pub fn convert_event(self, house_id: String) -> SaveHouseEvent {
        SaveHouseEvent {
            house_id,
            community_name: self.community_name,
            house_address: self.house_address,
            floor: self.floor,
            property: self.property,
            area: self.area,
            bedrooms: self.bedrooms,
            living_rooms: self.living_rooms,
            bathrooms: self.bathrooms,
            orientation: self.orientation,
            decoration_status: self.decoration_status,
            house_description: self.house_description,
            house_image: self.house_image,
            owner_name: self.owner_name,
            owner_phone: self.owner_phone,

            // 2024-07-24 00:00:00
            title: self.title,
            recommended_tags: self.recommended_tags,
            floor_range: self.floor_range,
            elevator: self.elevator,
            household: self.household,
            balcony: self.balcony,
            kitchen: self.kitchen,
            building_structure: self.building_structure,
            building_year: self.building_year,
            property_rights: self.property_rights,
            property_duration: self.property_duration,
            property_date: self.property_date,
            delivery_date: self.delivery_date,
            school_qualification: self.school_qualification,
            household_registration: self.household_registration,
            source: self.source,
            unique_house: self.unique_house,
            facilities: self.facilities,
            usable_area: self.usable_area,
            current_status: self.current_status,
            house_type: self.house_type,

            updated_by: self.updated_by,
            created_by: self.created_by,
        }
    }
}
