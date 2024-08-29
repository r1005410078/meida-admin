use crate::{
    infrastructure::{
        db::connection::DBPool,
        repositories::{
            entities::house_second_hand::{HouseSecondHandListed, HouseSecondHandSold},
            object_value::query_value::{BigDecimalRange, IntRange, TableData, YearRange},
        },
    },
    schema::{house_second_hand, house_second_hand_sold},
};
use bigdecimal::BigDecimal;
use chrono::NaiveDateTime;
use diesel::{
    dsl::{exists, select},
    prelude::AsChangeset,
    BoolExpressionMethods, ExpressionMethods, QueryDsl, RunQueryDsl, TextExpressionMethods,
};
use diesel::{prelude::Insertable, query_dsl::methods::SelectDsl, SelectableHelper};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, AsChangeset, Insertable)]
#[diesel(table_name = house_second_hand)]
pub struct SaveHouseSecondHandListedDto {
    pub house_id: String,
    pub community_name: String,
    pub pice: Option<BigDecimal>,
    pub low_pice: Option<BigDecimal>,
    pub listed: Option<i8>,
    pub listed_time: Option<NaiveDateTime>,
    pub unlisted_time: Option<NaiveDateTime>,
    pub comment: Option<String>,
    pub tags: Option<String>,

    // 2024-07-27 03:45:54
    pub down_payment: Option<BigDecimal>, // '首付' 记录首付金额，精度为两位小数
    pub viewing_method: Option<String>,   // '看房方式' 记录看房的方式（如预约、随时可看等）
    pub payment_method: Option<String>,   // '付款方式' 记录付款方式（如一次性付款、按揭贷款等）
    pub taxes_and_fees: Option<BigDecimal>, // '房源税费' 记录房源涉及的税费，精度为两位小数
    pub full_payment_required: Option<bool>, //  '是否全款'  标识是否必须全款，0 为否，1 为是
    pub urgent_sale: Option<bool>,        // '是否急切' 标识是否急切出售，0 为否，1 为是
}

impl SaveHouseSecondHandListedDto {
    pub async fn save(&self, pool: DBPool) -> Result<(), diesel::result::Error> {
        use crate::schema::house_second_hand::dsl::*;
        let mut conn = pool.get().unwrap();

        let existed: bool = select(exists(
            house_second_hand.filter(house_id.eq(&self.house_id)),
        ))
        .get_result(&mut conn)
        .expect("Error checking if house_second_hand exists");

        if !existed {
            diesel::insert_into(house_second_hand)
                .values(self)
                .execute(&mut conn)
                .expect("Error saving new house");
        } else {
            diesel::update(house_second_hand)
                .filter(house_id.eq(&self.house_id))
                .set(self)
                .execute(&mut conn)
                .expect("Error saving new house");
        }

        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Insertable)]
#[diesel(table_name = house_second_hand_sold)]
pub struct NewHouseSecondHandSoldDto {
    pub house_id: String,
    pub community_name: String,
    pub days_to_sell: i32,
    pub sold_price: BigDecimal,
    pub sold_time: Option<NaiveDateTime>,
}

impl NewHouseSecondHandSoldDto {
    pub async fn create(&self, pool: DBPool) -> Result<(), diesel::result::Error> {
        let mut conn = pool.get().unwrap();

        diesel::insert_into(house_second_hand_sold::table)
            .values(self)
            .execute(&mut conn)
            .expect("Error saving new house");

        Ok(())
    }
}

// 登记的二手房
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QueryHouseSecondHandDto {
    pub listed: Option<i8>,
    pub pice: Option<BigDecimalRange>,
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

impl QueryHouseSecondHandDto {
    pub fn list(&self, pool: DBPool) -> TableData<HouseSecondHandListed> {
        use crate::schema::house;
        use crate::schema::house_second_hand::dsl::*;
        use crate::schema::residential;
        use diesel::JoinOnDsl;

        let conn = &mut pool.get().unwrap();

        let get_query = || {
            let mut result = SelectDsl::select(
                house_second_hand
                    .inner_join(house::table.on(house::house_id.eq(house_id)))
                    .inner_join(
                        residential::table.on(residential::community_name.eq(community_name)),
                    ),
                HouseSecondHandListed::as_select(),
            )
            .into_boxed();

            if let Some(ref input_listed) = self.listed {
                result = result.filter(listed.eq(input_listed));
            }

            if let Some(ref input_pice) = self.pice {
                if let BigDecimalRange {
                    start: Some(input_pice),
                    end: None,
                } = input_pice
                {
                    result = result.filter(pice.ge(input_pice));
                } else if let BigDecimalRange {
                    start: None,
                    end: Some(input_pice),
                } = input_pice
                {
                    result = result.filter(pice.le(input_pice));
                } else if let BigDecimalRange {
                    start: Some(start),
                    end: Some(end),
                } = input_pice
                {
                    result = result.filter(pice.ge(start).and(pice.le(end)));
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

        let total = get_query()
            .count()
            .get_result(conn)
            .expect("Error loading house second hand");

        let page_index = self.page_index.unwrap_or(1);
        let page_size = self.page_size.unwrap_or(10);

        let result = get_query()
            .offset((page_index - 1) * page_size)
            .limit(page_size);

        let data = result
            .load::<HouseSecondHandListed>(conn)
            .expect("Error loading houses");

        TableData::new(data, total)
    }
}

// 出售的二手房
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QueryHouseSecondHandSoldDto {
    // 卖出的价格
    pub sold_price: Option<BigDecimal>,
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

impl QueryHouseSecondHandSoldDto {
    pub fn list(&self, pool: DBPool) -> TableData<HouseSecondHandSold> {
        use crate::schema::house;
        use crate::schema::house_second_hand_sold::dsl::*;
        use crate::schema::residential;
        use diesel::JoinOnDsl;

        let conn = &mut pool.get().unwrap();

        let get_query = || {
            let mut result = SelectDsl::select(
                house_second_hand_sold
                    .inner_join(house::table.on(house::house_id.eq(house_id)))
                    .inner_join(
                        residential::table.on(residential::community_name.eq(community_name)),
                    ),
                HouseSecondHandSold::as_select(),
            )
            .into_boxed();

            if let Some(ref input_pice) = self.sold_price {
                result = result.filter(sold_price.ge(input_pice));
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

        let total = get_query()
            .count()
            .get_result(conn)
            .expect("Error loading house second hand");

        let page_index = self.page_index.unwrap_or(1);
        let page_size = self.page_size.unwrap_or(10);

        let result = get_query()
            .offset((page_index - 1) * page_size)
            .limit(page_size);

        let data = result
            .load::<HouseSecondHandSold>(conn)
            .expect("Error loading houses");

        TableData::new(data, total)
    }
}
