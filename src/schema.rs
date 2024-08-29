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
        property -> Varchar,
        #[max_length = 255]
        decoration_status -> Nullable<Varchar>,
        area -> Decimal,
        bedrooms -> Integer,
        living_rooms -> Integer,
        bathrooms -> Integer,
        #[max_length = 20]
        orientation -> Nullable<Varchar>,
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
        floor -> Integer,
        #[max_length = 255]
        floor_range -> Nullable<Varchar>,
        #[max_length = 255]
        title -> Varchar,
        #[max_length = 255]
        recommended_tags -> Varchar,
        elevator -> Nullable<Integer>,
        household -> Nullable<Integer>,
        balcony -> Nullable<Integer>,
        kitchen -> Nullable<Integer>,
        #[max_length = 100]
        building_structure -> Nullable<Varchar>,
        building_year -> Nullable<Date>,
        #[max_length = 100]
        property_rights -> Nullable<Varchar>,
        property_duration -> Nullable<Integer>,
        property_date -> Nullable<Date>,
        delivery_date -> Nullable<Date>,
        #[max_length = 50]
        school_qualification -> Nullable<Varchar>,
        #[max_length = 50]
        household_registration -> Nullable<Varchar>,
        unique_house -> Nullable<Bool>,
        facilities -> Nullable<Text>,
        usable_area -> Decimal,
        #[max_length = 255]
        current_status -> Nullable<Varchar>,
        #[max_length = 255]
        house_type -> Nullable<Varchar>,
        #[max_length = 100]
        source -> Nullable<Varchar>,
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
        rental_start_time -> Nullable<Datetime>,
        rental_end_time -> Nullable<Datetime>,
        created_at -> Datetime,
        updated_at -> Datetime,
    }
}

diesel::table! {
    house_rental (house_id) {
        #[max_length = 255]
        house_id -> Varchar,
        #[max_length = 50]
        community_name -> Varchar,
        rent_pice -> Decimal,
        rent_low_pice -> Nullable<Decimal>,
        listed -> Tinyint,
        listed_time -> Nullable<Datetime>,
        unlisted_time -> Nullable<Datetime>,
        comment -> Text,
        tags -> Text,
        #[max_length = 100]
        viewing_method -> Nullable<Varchar>,
        #[max_length = 100]
        payment_method -> Nullable<Varchar>,
        full_payment_required -> Nullable<Bool>,
        urgent_sale -> Nullable<Bool>,
        created_at -> Datetime,
        updated_at -> Datetime,
    }
}

diesel::table! {
    house_rental_sold (sold_id) {
        sold_id -> Integer,
        #[max_length = 255]
        house_id -> Varchar,
        #[max_length = 50]
        community_name -> Varchar,
        rent_pice -> Decimal,
        rent_start_time -> Datetime,
        rent_end_time -> Datetime,
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
        comment -> Text,
        tags -> Text,
        down_payment -> Nullable<Decimal>,
        #[max_length = 100]
        viewing_method -> Nullable<Varchar>,
        #[max_length = 100]
        payment_method -> Nullable<Varchar>,
        taxes_and_fees -> Nullable<Decimal>,
        full_payment_required -> Nullable<Bool>,
        urgent_sale -> Nullable<Bool>,
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

diesel::table! {
    users (id) {
        #[max_length = 36]
        id -> Char,
        #[max_length = 50]
        username -> Varchar,
        #[max_length = 255]
        password_hash -> Varchar,
        #[max_length = 100]
        phone -> Varchar,
        #[max_length = 255]
        avatar -> Nullable<Varchar>,
        is_active -> Nullable<Bool>,
        #[max_length = 20]
        role -> Nullable<Varchar>,
        created_at -> Nullable<Timestamp>,
        updated_at -> Nullable<Timestamp>,
    }
}

diesel::allow_tables_to_appear_in_same_query!(
    house,
    house_aggregate,
    house_rental,
    house_rental_sold,
    house_second_hand,
    house_second_hand_sold,
    residential,
    residential_aggregate,
    users,
);
