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
        property -> Nullable<Varchar>,
        #[max_length = 255]
        decoration_status -> Nullable<Varchar>,
        area -> Decimal,
        bedrooms -> Integer,
        living_rooms -> Integer,
        bathrooms -> Integer,
        balcony -> Nullable<Integer>,
        kitchen -> Nullable<Integer>,
        #[max_length = 20]
        orientation -> Nullable<Varchar>,
        house_description -> Nullable<Text>,
        house_image -> Nullable<Text>,
        #[max_length = 100]
        owner_name -> Varchar,
        #[max_length = 20]
        owner_phone -> Varchar,
        floor -> Nullable<Integer>,
        #[max_length = 255]
        floor_range -> Nullable<Varchar>,
        #[max_length = 255]
        title -> Varchar,
        #[max_length = 255]
        recommended_tags -> Nullable<Varchar>,
        elevator -> Nullable<Integer>,
        household -> Nullable<Integer>,
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
        usable_area -> Nullable<Decimal>,
        #[max_length = 255]
        current_status -> Nullable<Varchar>,
        #[max_length = 255]
        house_type -> Nullable<Varchar>,
        #[max_length = 100]
        source -> Nullable<Varchar>,
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
        rental_start_time -> Nullable<Datetime>,
        rental_end_time -> Nullable<Datetime>,
        #[max_length = 255]
        created_by -> Nullable<Varchar>,
        #[max_length = 255]
        updated_by -> Nullable<Varchar>,
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
        comment -> Nullable<Text>,
        tags -> Nullable<Text>,
        #[max_length = 100]
        viewing_method -> Nullable<Varchar>,
        #[max_length = 100]
        payment_method -> Nullable<Varchar>,
        full_payment_required -> Nullable<Bool>,
        urgent_sale -> Nullable<Bool>,
        #[max_length = 255]
        created_by -> Nullable<Varchar>,
        #[max_length = 255]
        updated_by -> Nullable<Varchar>,
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
        #[max_length = 255]
        created_by -> Nullable<Varchar>,
        #[max_length = 255]
        updated_by -> Nullable<Varchar>,
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
        comment -> Nullable<Text>,
        tags -> Nullable<Text>,
        down_payment -> Nullable<Decimal>,
        #[max_length = 100]
        viewing_method -> Nullable<Varchar>,
        #[max_length = 100]
        payment_method -> Nullable<Varchar>,
        taxes_and_fees -> Nullable<Decimal>,
        full_payment_required -> Nullable<Bool>,
        urgent_sale -> Nullable<Bool>,
        #[max_length = 255]
        created_by -> Nullable<Varchar>,
        #[max_length = 255]
        updated_by -> Nullable<Varchar>,
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
        #[max_length = 255]
        created_by -> Nullable<Varchar>,
        #[max_length = 255]
        updated_by -> Nullable<Varchar>,
        created_at -> Datetime,
        updated_at -> Datetime,
    }
}

diesel::table! {
    imports_properties (id) {
        #[max_length = 255]
        id -> Varchar,
        #[max_length = 255]
        pice_type -> Varchar,
        #[max_length = 255]
        usage -> Varchar,
        #[max_length = 255]
        platform -> Varchar,
        #[max_length = 255]
        file_status -> Varchar,
        #[max_length = 255]
        file_error -> Nullable<Varchar>,
        #[max_length = 255]
        file_name -> Varchar,
        #[max_length = 255]
        file_path -> Varchar,
        file_size -> Bigint,
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
        year_built -> Nullable<Smallint>,
        #[max_length = 100]
        community_type -> Varchar,
        #[max_length = 100]
        property_management_company -> Nullable<Varchar>,
        description -> Nullable<Text>,
        #[max_length = 255]
        created_by -> Nullable<Varchar>,
        #[max_length = 255]
        updated_by -> Nullable<Varchar>,
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
        #[max_length = 255]
        created_by -> Nullable<Varchar>,
        #[max_length = 255]
        updated_by -> Nullable<Varchar>,
        created_at -> Nullable<Datetime>,
        updated_at -> Nullable<Datetime>,
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
        #[max_length = 255]
        created_by -> Nullable<Varchar>,
        #[max_length = 255]
        updated_by -> Nullable<Varchar>,
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
    imports_properties,
    residential,
    residential_aggregate,
    users,
);
