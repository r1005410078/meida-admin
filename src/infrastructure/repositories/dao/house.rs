use crate::domain::houses::entities::house::HousePO;
use crate::infrastructure::db::connection::DBPool;
use crate::infrastructure::repositories::object_value::query_value::{
    BigDecimalRange, IntRange, TableData,
};
use chrono::NaiveDateTime;
use diesel::{BoolExpressionMethods, RunQueryDsl};
use diesel::{ExpressionMethods, QueryDsl, TextExpressionMethods};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QueryHouseDao {
    pub community_name: Option<String>,
    pub house_address: Option<String>,
    pub floor: Option<IntRange>,
    pub property: Option<String>,
    pub house_age: Option<NaiveDateTime>,
    pub area: Option<BigDecimalRange>,
    pub bedrooms: Option<IntRange>,
    pub living_rooms: Option<i32>,
    pub bathrooms: Option<i32>,
    pub orientation: Option<String>,
    pub decoration_status: Option<String>,
    pub house_description: Option<String>,
    pub owner_name: Option<String>,
    pub owner_phone: Option<String>,

    // 分页
    pub page_index: Option<i64>,
    pub page_size: Option<i64>,
}

impl QueryHouseDao {
    pub fn list(&self, pool: DBPool) -> TableData<HousePO> {
        use crate::schema::house::dsl::*;
        let conn = &mut pool.get().unwrap();

        let get_query = || {
            let mut query = house.into_boxed();

            if let Some(ref input_community_name) = self.community_name {
                query = query.filter(community_name.eq(input_community_name));
            }

            if let Some(ref input_house_address) = self.house_address {
                query = query.filter(house_address.like(format!("%{}%", input_house_address)));
            }

            if let Some(ref input_floor) = self.floor {
                if let IntRange {
                    start: Some(input_floor),
                    end: None,
                } = input_floor
                {
                    query = query.filter(floor.ge(input_floor));
                } else if let IntRange {
                    start: None,
                    end: Some(input_floor),
                } = input_floor
                {
                    query = query.filter(floor.le(input_floor));
                } else if let IntRange {
                    start: Some(start),
                    end: Some(end),
                } = input_floor
                {
                    query = query.filter(floor.ge(start).and(floor.le(end)));
                }
            }

            if let Some(ref input_house_age) = self.house_age {
                query = query.filter(house_age.ge(input_house_age));
            }

            if let Some(ref input_property) = self.property {
                query = query.filter(property.eq(input_property));
            }

            if let Some(ref input_area) = self.area {
                if let BigDecimalRange {
                    start: Some(input_area),
                    end: None,
                } = input_area
                {
                    query = query.filter(area.ge(input_area));
                } else if let BigDecimalRange {
                    start: None,
                    end: Some(input_area),
                } = input_area
                {
                    query = query.filter(area.le(input_area));
                } else if let BigDecimalRange {
                    start: Some(start),
                    end: Some(end),
                } = input_area
                {
                    query = query.filter(area.ge(start).and(area.le(end)));
                }
            }

            if let Some(ref input_bedrooms) = self.bedrooms {
                if let IntRange {
                    start: Some(input_bedrooms),
                    end: None,
                } = input_bedrooms
                {
                    query = query.filter(bedrooms.ge(input_bedrooms));
                } else if let IntRange {
                    start: None,
                    end: Some(input_bedrooms),
                } = input_bedrooms
                {
                    query = query.filter(bedrooms.le(input_bedrooms));
                } else if let IntRange {
                    start: Some(start),
                    end: Some(end),
                } = input_bedrooms
                {
                    query = query.filter(bedrooms.ge(start).and(bedrooms.le(end)));
                }
            }

            if let Some(ref input_living_rooms) = self.living_rooms {
                query = query.filter(living_rooms.ge(input_living_rooms));
            }

            if let Some(ref input_bathrooms) = self.bathrooms {
                query = query.filter(bathrooms.ge(input_bathrooms));
            }

            if let Some(ref input_orientation) = self.orientation {
                query = query.filter(orientation.eq(input_orientation));
            }

            if let Some(ref input_decoration_status) = self.decoration_status {
                query = query.filter(decoration_status.eq(input_decoration_status));
            }

            if let Some(ref input_house_description) = self.house_description {
                query =
                    query.filter(house_description.like(format!("%{}%", input_house_description)));
            }

            if let Some(ref input_owner_name) = self.owner_name {
                query = query.filter(owner_name.eq(input_owner_name));
            }

            if let Some(ref input_owner_phone) = self.owner_phone {
                query = query.filter(owner_phone.eq(input_owner_phone));
            }

            query
        };

        let page_index = self.page_index.unwrap_or(1);
        let page_size = self.page_size.unwrap_or(10);
        let data = get_query()
            .offset((page_index - 1) * page_size)
            .limit(page_size)
            .load::<HousePO>(conn)
            .expect("Error loading houses");

        let total = get_query()
            .count()
            .get_result(conn)
            .expect("Error loading house second hand");

        TableData::new(data, total)
    }
}
