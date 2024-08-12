use crate::domain::houses::entities::house::HousePO;
use crate::domain::houses::events::house::NewHouseEvent;
use crate::infrastructure::db::connection::DBPool;
use crate::infrastructure::repositories::object_value::query_value::TableData;
use crate::schema::house;
use bigdecimal::BigDecimal;
use diesel::dsl::count_star;
use diesel::prelude::Insertable;
use diesel::{ExpressionMethods, QueryDsl, TextExpressionMethods};
use diesel::{RunQueryDsl, Selectable};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, Insertable, Selectable)]
#[diesel(table_name = house)]
pub struct NewHouseDto {
    house_id: String,
    community_name: String,
    house_address: String,
    house_type: String,
    area: BigDecimal,
    bedrooms: i32,
    living_rooms: i32,
    bathrooms: i32,
    orientation: String,
    decoration_status: String,
    status: String,
    house_description: String,
    house_image: String,
    owner_name: String,
    owner_phone: String,
    created_by: String,
    updated_by: String,
}

impl From<NewHouseEvent> for NewHouseDto {
    fn from(event: NewHouseEvent) -> Self {
        NewHouseDto {
            house_id: event.house_id,
            community_name: event.community_name,
            house_address: event.house_address,
            house_type: event.house_type,
            area: event.area,
            bedrooms: event.bedrooms,
            living_rooms: event.living_rooms,
            bathrooms: event.bathrooms,
            orientation: event.orientation.clone(),
            decoration_status: event.decoration_status.clone(),
            status: event.status.clone(),
            house_description: event.house_description.clone(),
            house_image: event.house_image.clone(),
            owner_name: event.owner_name.clone(),
            owner_phone: event.owner_phone.clone(),
            created_by: event.created_by.clone(),
            updated_by: event.updated_by.clone(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QueryHouseDao {
    pub community_name: Option<String>,
    pub house_address: Option<String>,
    pub house_type: Option<String>,
    pub area: Option<BigDecimal>,
    pub bedrooms: Option<i32>,
    pub living_rooms: Option<i32>,
    pub bathrooms: Option<i32>,
    pub orientation: Option<String>,
    pub decoration_status: Option<String>,
    pub status: Option<String>,
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

        let mut query = house.into_boxed();

        if let Some(ref input_community_name) = self.community_name {
            query = query.filter(community_name.eq(input_community_name));
        }

        if let Some(ref input_house_address) = self.house_address {
            query = query.filter(house_address.eq(input_house_address));
        }

        if let Some(ref input_house_type) = self.house_type {
            query = query.filter(house_type.eq(input_house_type));
        }

        if let Some(ref input_area) = self.area {
            query = query.filter(area.ge(input_area));
        }

        if let Some(ref input_bedrooms) = self.bedrooms {
            query = query.filter(bedrooms.ge(input_bedrooms));
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

        if let Some(ref input_status) = self.status {
            query = query.filter(status.eq(input_status));
        }

        if let Some(ref input_house_description) = self.house_description {
            query = query.filter(house_description.like(format!("%{}%", input_house_description)));
        }

        if let Some(ref input_owner_name) = self.owner_name {
            query = query.filter(owner_name.eq(input_owner_name));
        }

        if let Some(ref input_owner_phone) = self.owner_phone {
            query = query.filter(owner_phone.eq(input_owner_phone));
        }

        let page_index = self.page_index.unwrap_or(1);
        let page_size = self.page_size.unwrap_or(10);

        query = query.offset((page_index - 1) * page_size).limit(page_size);

        let data = query.load::<HousePO>(conn).expect("Error loading houses");

        let total = house
            .select(count_star())
            .first(conn)
            .expect("Error loading houses");

        TableData::new(data, total)
    }
}
