use bigdecimal::BigDecimal;
use chrono::NaiveDateTime;
use diesel::{
    dsl::{exists, sql},
    prelude::{AsChangeset, Insertable},
    query_dsl::methods::SelectDsl,
    select, BoolExpressionMethods, ExpressionMethods, QueryDsl, RunQueryDsl,
};
use diesel::{SelectableHelper, TextExpressionMethods};
use serde::{Deserialize, Serialize};

use crate::infrastructure::repositories::object_value::query_value::{
    BigDecimalRange, IntRange, YearRange,
};
use crate::infrastructure::repositories::{
    entities::rental_house::RentalHouseSold,
};
use crate::schema::house_rental_sold;
use crate::{
    domain::houses::events::rental_house::SaveRentalHouseEvent,
    infrastructure::{
        db::connection::DBPool, repositories::entities::rental_house::RentalHouseListed,
    },
    schema::house_rental,
};
use crate::{
    domain::houses::events::rental_house::{
        RentalHouseListedEvent, RentalHouseSoldEvent, RentalHouseUnListedEvent,
    },
    infrastructure::repositories::object_value::query_value::TableData,
};

#[derive(Debug, Insertable, AsChangeset, Serialize, Deserialize)]
#[diesel(table_name = house_rental)]
pub struct SaveRentalHouseDao {
    pub house_id: String,
    pub community_name: Option<String>,
    pub listed: i8,
    pub rent_pice: Option<BigDecimal>,
    pub rent_low_pice: Option<BigDecimal>,
    pub comment: Option<String>,
    pub tags: Option<String>,
}

impl SaveRentalHouseDao {
    pub fn save(&self, pool: DBPool) -> Result<(), diesel::result::Error> {
        use crate::schema::house_rental::dsl::*;
        let conn = &mut pool.get().unwrap();

        let sean_exists: bool = select(exists(
            house_rental.filter(house_id.eq(self.house_id.clone())),
        ))
        .get_result(conn)
        .expect("Error loading houses");

        if sean_exists {
            diesel::update(house_rental.filter(house_id.eq(self.house_id.clone())))
                .set(self)
                .execute(conn)?;
        } else {
            diesel::insert_into(house_rental)
                .values(self)
                .execute(conn)?;
        }

        Ok(())
    }
}

impl From<SaveRentalHouseEvent> for SaveRentalHouseDao {
    fn from(event: SaveRentalHouseEvent) -> Self {
        Self {
            house_id: event.house_id,
            listed: 1,
            community_name: Some(event.community_name),
            rent_pice: Some(event.rent_pice),
            rent_low_pice: event.rent_low_pice,
            comment: Some(event.comment),
            tags: Some(event.tags),
        }
    }
}

impl From<RentalHouseListedEvent> for SaveRentalHouseDao {
    fn from(event: RentalHouseListedEvent) -> Self {
        Self {
            house_id: event.house_id,
            listed: event.listed,
            community_name: None,
            rent_pice: None,
            rent_low_pice: None,
            comment: None,
            tags: None,
        }
    }
}

impl From<RentalHouseUnListedEvent> for SaveRentalHouseDao {
    fn from(event: RentalHouseUnListedEvent) -> Self {
        Self {
            house_id: event.house_id,
            listed: event.listed,
            community_name: None,
            rent_pice: None,
            rent_low_pice: None,
            comment: None,
            tags: None,
        }
    }
}

// 出租的房源
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QueryRentalHouseListedDto {
    pub listed: Option<i8>,
    pub rent_pice: Option<BigDecimalRange>,
    pub tags: Option<String>,
    pub comment: Option<String>,

    // 房源
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
    pub status: Option<String>,
    pub house_description: Option<String>,
    pub owner_name: Option<String>,
    pub owner_phone: Option<String>,

    // 小区
    pub community_name: Option<String>,
    pub community_type: Option<String>,
    pub region: Option<String>,
    pub year_built: Option<YearRange>,

    // 分页
    pub page_index: Option<i64>,
    pub page_size: Option<i64>,
}

impl QueryRentalHouseListedDto {
    pub fn list(&self, pool: DBPool) -> TableData<RentalHouseListed> {
        use crate::schema::house;
        use crate::schema::house_rental::dsl::*;
        use crate::schema::residential;
        use diesel::JoinOnDsl;

        let conn = &mut pool.get().unwrap();

        let get_query = || {
            let mut result = SelectDsl::select(
                house_rental
                    .inner_join(house::table.on(house::house_id.eq(house_id)))
                    .inner_join(
                        residential::table.on(residential::community_name.eq(community_name)),
                    ),
                RentalHouseListed::as_select(),
            )
            .into_boxed();

            if let Some(ref _listed) = self.listed {
                result = result.filter(listed.eq(_listed));
            }

            if let Some(ref input_rent_pice) = self.rent_pice {
                if let BigDecimalRange {
                    start: Some(input_rent_pice),
                    end: None,
                } = input_rent_pice
                {
                    result = result.filter(rent_pice.ge(input_rent_pice));
                } else if let BigDecimalRange {
                    start: None,
                    end: Some(input_rent_pice),
                } = input_rent_pice
                {
                    result = result.filter(rent_pice.le(input_rent_pice));
                } else if let BigDecimalRange {
                    start: Some(start),
                    end: Some(end),
                } = input_rent_pice
                {
                    result = result.filter(rent_pice.ge(start).and(rent_pice.le(end)));
                }
            }

            if let Some(ref input_comment) = self.comment {
                result = result.filter(comment.like(format!("%{}%", input_comment)));
            }

            ////////// 房源
            if let Some(ref input_address) = self.house_address {
                result = result.filter(house::house_address.like(format!("%{}%", input_address)));
            }

            if let Some(ref input_floor) = self.floor {
                if let IntRange {
                    start: Some(input_floor),
                    end: None,
                } = input_floor
                {
                    result = result.filter(house::floor.ge(input_floor));
                } else if let IntRange {
                    start: None,
                    end: Some(input_floor),
                } = input_floor
                {
                    result = result.filter(house::floor.le(input_floor));
                } else if let IntRange {
                    start: Some(start),
                    end: Some(end),
                } = input_floor
                {
                    result = result.filter(house::floor.ge(start).and(house::floor.le(end)));
                }
            }

            if let Some(ref input_property) = self.property {
                result = result.filter(house::property.eq(input_property));
            }

            if let Some(ref input_area) = self.area {
                if let BigDecimalRange {
                    start: Some(input_area),
                    end: None,
                } = input_area
                {
                    result = result.filter(house::area.ge(input_area));
                } else if let BigDecimalRange {
                    start: None,
                    end: Some(input_area),
                } = input_area
                {
                    result = result.filter(house::area.le(input_area));
                } else if let BigDecimalRange {
                    start: Some(start),
                    end: Some(end),
                } = input_area
                {
                    result = result.filter(house::area.ge(start).and(house::area.le(end)));
                }
            }

            if let Some(ref input_bedrooms) = self.bedrooms {
                if let IntRange {
                    start: Some(input_bedrooms),
                    end: None,
                } = input_bedrooms
                {
                    result = result.filter(house::bedrooms.ge(input_bedrooms));
                } else if let IntRange {
                    start: None,
                    end: Some(input_bedrooms),
                } = input_bedrooms
                {
                    result = result.filter(house::bedrooms.le(input_bedrooms));
                } else if let IntRange {
                    start: Some(start),
                    end: Some(end),
                } = input_bedrooms
                {
                    result = result.filter(house::bedrooms.ge(start).and(house::bedrooms.le(end)));
                }
            }

            if let Some(ref input_living_rooms) = self.living_rooms {
                result = result.filter(house::living_rooms.ge(input_living_rooms));
            }

            if let Some(ref input_bathrooms) = self.bathrooms {
                result = result.filter(house::bathrooms.ge(input_bathrooms));
            }

            if let Some(ref input_orientation) = self.orientation {
                result = result.filter(house::orientation.eq(input_orientation));
            }

            if let Some(ref input_decoration_status) = self.decoration_status {
                result = result.filter(house::decoration_status.eq(input_decoration_status));
            }

            if let Some(ref input_house_description) = self.house_description {
                result = result.filter(
                    house::house_description.like(format!("%{}%", input_house_description)),
                );
            }

            if let Some(ref input_owner_name) = self.owner_name {
                result = result.filter(house::owner_name.like(format!("%{}%", input_owner_name)));
            }

            if let Some(ref input_owner_phone) = self.owner_phone {
                result = result.filter(house::owner_phone.like(format!("%{}%", input_owner_phone)));
            }

            if let Some(ref input_community_name) = self.community_name {
                result = result.filter(residential::community_name.eq(input_community_name));
            }

            if let Some(ref input_community_type) = self.community_type {
                result = result.filter(residential::community_type.like(input_community_type));
            }

            if let Some(ref input_region) = self.region {
                result = result.filter(residential::region.eq(input_region));
            }

            result
        };

        // 分页
        let page_index = self.page_index.unwrap_or(1);
        let page_size = self.page_size.unwrap_or(10);

        let total = get_query()
            .count()
            .get_result(conn)
            .expect("Error loading rental house");

        let data = get_query()
            .offset((page_index - 1) * page_size)
            .limit(page_size)
            .load::<RentalHouseListed>(conn)
            .expect("Error loading houses");

        TableData::new(data, total)
    }
}

// 已出租的房源
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QueryRentalHouseSoldDto {
    pub rent_pice: Option<BigDecimal>,
    // 房源
    pub house_address: Option<String>,
    pub floor: Option<i32>,
    pub property: Option<String>,
    pub house_age: Option<NaiveDateTime>,
    pub area: Option<BigDecimal>,
    pub bedrooms: Option<i32>,
    pub living_rooms: Option<i32>,
    pub bathrooms: Option<i32>,
    pub orientation: Option<String>,
    pub decoration_status: Option<String>,
    pub house_description: Option<String>,
    pub owner_name: Option<String>,
    pub owner_phone: Option<String>,

    // 小区
    pub community_name: Option<String>,
    pub community_type: Option<String>,
    pub region: Option<String>,
    pub year_built: Option<YearRange>,

    // 分页
    pub page_index: Option<i64>,
    pub page_size: Option<i64>,
}

impl QueryRentalHouseSoldDto {
    pub fn list(&self, pool: DBPool) -> TableData<RentalHouseSold> {
        use crate::schema::house;
        use crate::schema::house_rental_sold::dsl::*;
        use crate::schema::residential;
        use diesel::JoinOnDsl;

        let conn = &mut pool.get().unwrap();

        let get_query = || {
            let mut result = SelectDsl::select(
                house_rental_sold
                    .inner_join(house::table.on(house::house_id.eq(house_id)))
                    .inner_join(
                        residential::table.on(residential::community_name.eq(community_name)),
                    ),
                RentalHouseSold::as_select(),
            )
            .into_boxed();

            if let Some(ref _rent_pice) = self.rent_pice {
                result = result.filter(rent_pice.ge(_rent_pice));
            }

            ////////// 房源
            if let Some(ref input_address) = self.house_address {
                result = result.filter(house::house_address.like(format!("%{}%", input_address)));
            }

            if let Some(ref input_floor) = self.floor {
                result = result.filter(house::floor.ge(input_floor));
            }

            if let Some(ref input_property) = self.property {
                result = result.filter(house::property.eq(input_property));
            }

            if let Some(ref input_area) = self.area {
                result = result.filter(house::area.ge(input_area));
            }

            if let Some(ref input_bedrooms) = self.bedrooms {
                result = result.filter(house::bedrooms.ge(input_bedrooms));
            }

            if let Some(ref input_living_rooms) = self.living_rooms {
                result = result.filter(house::living_rooms.ge(input_living_rooms));
            }

            if let Some(ref input_bathrooms) = self.bathrooms {
                result = result.filter(house::bathrooms.ge(input_bathrooms));
            }

            if let Some(ref input_orientation) = self.orientation {
                result = result.filter(house::orientation.eq(input_orientation));
            }

            if let Some(ref input_decoration_status) = self.decoration_status {
                result = result.filter(house::decoration_status.eq(input_decoration_status));
            }

            if let Some(ref input_house_description) = self.house_description {
                result = result.filter(
                    house::house_description.like(format!("%{}%", input_house_description)),
                );
            }

            if let Some(ref input_owner_name) = self.owner_name {
                result = result.filter(house::owner_name.like(format!("%{}%", input_owner_name)));
            }

            if let Some(ref input_owner_phone) = self.owner_phone {
                result = result.filter(house::owner_phone.like(format!("%{}%", input_owner_phone)));
            }

            if let Some(ref input_community_name) = self.community_name {
                result = result.filter(residential::community_name.eq(input_community_name));
            }

            if let Some(ref input_community_type) = self.community_type {
                result = result.filter(residential::community_type.like(input_community_type));
            }

            if let Some(ref input_region) = self.region {
                result = result.filter(residential::region.eq(input_region));
            }

            result
        };

        // 分页
        let page_index = self.page_index.unwrap_or(1);
        let page_size = self.page_size.unwrap_or(10);

        let total = get_query()
            .count()
            .get_result(conn)
            .expect("Error loading rental house");

        let data = get_query()
            .offset((page_index - 1) * page_size)
            .limit(page_size)
            .load::<RentalHouseSold>(conn)
            .expect("Error loading houses");

        TableData::new(data, total)
    }
}

#[derive(Debug, Insertable, AsChangeset, Serialize, Deserialize)]
#[diesel(table_name = house_rental_sold)]
pub struct RentalHouseSoldDao {
    pub house_id: String,
    pub community_name: String,
    pub rent_pice: BigDecimal,
    pub rent_start_time: NaiveDateTime,
    pub rent_end_time: NaiveDateTime,
}

impl RentalHouseSoldDao {
    pub fn save(&self, pool: DBPool) -> Result<(), diesel::result::Error> {
        use crate::schema::house_rental_sold::dsl::*;
        let conn = &mut pool.get().unwrap();
        diesel::insert_into(house_rental_sold)
            .values(self)
            .execute(conn)?;
        Ok(())
    }
}

impl From<RentalHouseSoldEvent> for RentalHouseSoldDao {
    fn from(event: RentalHouseSoldEvent) -> Self {
        Self {
            house_id: event.house_id,
            community_name: event.community_name,
            rent_pice: event.rent_pice,
            rent_start_time: event.rent_start_time,
            rent_end_time: event.rent_end_time,
        }
    }
}
