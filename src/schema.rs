// @generated automatically by Diesel CLI.

diesel::table! {
    house (house_id) {
        #[max_length = 255]
        house_id -> Varchar,
        #[max_length = 255]
        community_name -> Varchar,
        #[max_length = 255]
        house_address -> Varchar,
        #[max_length = 50]
        house_type -> Varchar,
        area -> Decimal,
        bedrooms -> Integer,
        living_rooms -> Integer,
        bathrooms -> Integer,
        #[max_length = 20]
        orientation -> Nullable<Varchar>,
        #[max_length = 255]
        decoration_status -> Nullable<Varchar>,
        #[max_length = 50]
        status -> Nullable<Varchar>,
        house_description -> Nullable<Text>,
        #[max_length = 255]
        house_image -> Nullable<Varchar>,
        #[max_length = 100]
        owner_name -> Varchar,
        #[max_length = 20]
        owner_phone -> Varchar,
        #[max_length = 255]
        created_by -> Nullable<Varchar>,
        #[max_length = 255]
        updated_by -> Nullable<Varchar>,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    house_aggregate (house_id) {
        #[max_length = 255]
        house_id -> Varchar,
        #[max_length = 255]
        house_address -> Varchar,
        #[max_length = 50]
        community_name -> Varchar,
        registration_time -> Nullable<Datetime>,
        delete_time -> Nullable<Datetime>,
        second_hand_sale_time -> Nullable<Datetime>,
        second_hand_listed_time -> Nullable<Datetime>,
        second_hand_unlisted_time -> Nullable<Datetime>,
        rental_listed_time -> Nullable<Datetime>,
        rental_unlisted_time -> Nullable<Datetime>,
        rental_sale_time -> Nullable<Datetime>,
        rental_end_time -> Nullable<Datetime>,
        created_at -> Datetime,
        updated_at -> Datetime,
    }
}

diesel::table! {
    house_second_hand (house_id) {
        #[max_length = 255]
        house_id -> Varchar,
        #[max_length = 50]
        community_name -> Varchar,
        pice -> Decimal,
        low_pice -> Nullable<Decimal>,
        listed -> Tinyint,
        listed_time -> Nullable<Datetime>,
        unlisted_time -> Nullable<Datetime>,
        created_at -> Datetime,
        updated_at -> Datetime,
    }
}

diesel::table! {
    house_second_hand_sold (sold_id) {
        sold_id -> Integer,
        #[max_length = 255]
        house_id -> Varchar,
        #[max_length = 50]
        community_name -> Varchar,
        days_to_sell -> Integer,
        sold_price -> Decimal,
        sold_time -> Datetime,
        created_at -> Datetime,
        updated_at -> Datetime,
    }
}

diesel::table! {
    residential (community_name) {
        #[max_length = 255]
        community_name -> Varchar,
        #[max_length = 255]
        region -> Varchar,
        #[max_length = 100]
        city -> Varchar,
        #[max_length = 100]
        state -> Varchar,
        #[max_length = 20]
        postal_code -> Varchar,
        year_built -> Smallint,
        #[max_length = 100]
        community_type -> Varchar,
        #[max_length = 100]
        property_management_company -> Varchar,
        description -> Nullable<Text>,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    residential_aggregate (community_name) {
        #[max_length = 255]
        community_name -> Varchar,
        #[max_length = 255]
        region -> Varchar,
        #[max_length = 255]
        city -> Varchar,
        #[max_length = 255]
        state -> Varchar,
    }
}

diesel::allow_tables_to_appear_in_same_query!(
    house,
    house_aggregate,
    house_second_hand,
    house_second_hand_sold,
    residential,
    residential_aggregate,
);
