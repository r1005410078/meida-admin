// @generated automatically by Diesel CLI.

diesel::table! {
    delete_house (id) {
        id -> Integer,
        #[max_length = 255]
        house_id -> Nullable<Varchar>,
        #[max_length = 255]
        deleted_by -> Varchar,
        event_time -> Nullable<Timestamp>,
    }
}

diesel::table! {
    house (id) {
        id -> Integer,
        #[max_length = 255]
        house_id -> Varchar,
        #[max_length = 255]
        community_id -> Varchar,
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
        #[max_length = 50]
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
        event_time -> Nullable<Timestamp>,
    }
}

diesel::table! {
    house_aggregate (house_id) {
        #[max_length = 255]
        house_id -> Varchar,
        #[max_length = 255]
        house_address -> Varchar,
        #[max_length = 50]
        community_id -> Varchar,
        registration_time -> Nullable<Datetime>,
        delete_time -> Nullable<Datetime>,
        second_hand_sale_time -> Nullable<Datetime>,
        second_hand_listed_time -> Nullable<Datetime>,
        second_hand_unlisted_time -> Nullable<Datetime>,
        rental_listed_time -> Nullable<Datetime>,
        rental_unlisted_time -> Nullable<Datetime>,
        rental_sale_time -> Nullable<Datetime>,
        rental_end_time -> Nullable<Datetime>,
    }
}

diesel::table! {
    house_second_hand_listed (id) {
        id -> Integer,
        #[max_length = 255]
        house_id -> Varchar,
        pice -> Decimal,
        low_pice -> Nullable<Decimal>,
        event_time -> Nullable<Timestamp>,
    }
}

diesel::table! {
    house_second_hand_sale (id) {
        id -> Integer,
        sale_price -> Decimal,
        #[max_length = 255]
        house_id -> Varchar,
        event_time -> Nullable<Timestamp>,
    }
}

diesel::table! {
    house_second_hand_unlisted (id) {
        id -> Integer,
        #[max_length = 255]
        house_id -> Varchar,
        event_time -> Nullable<Timestamp>,
    }
}

diesel::table! {
    residential (community_id) {
        #[max_length = 255]
        community_id -> Varchar,
        #[max_length = 255]
        name -> Varchar,
        #[max_length = 255]
        address -> Varchar,
        #[max_length = 100]
        city -> Varchar,
        #[max_length = 100]
        state -> Varchar,
        #[max_length = 20]
        postal_code -> Varchar,
        year_built -> Smallint,
        #[max_length = 100]
        community_type -> Varchar,
        description -> Nullable<Text>,
    }
}

diesel::table! {
    residential_aggregate (community_id) {
        #[max_length = 255]
        community_id -> Varchar,
        #[max_length = 255]
        name -> Varchar,
        #[max_length = 255]
        address -> Varchar,
        #[max_length = 255]
        city -> Varchar,
        #[max_length = 255]
        state -> Varchar,
    }
}

diesel::allow_tables_to_appear_in_same_query!(
    delete_house,
    house,
    house_aggregate,
    house_second_hand_listed,
    house_second_hand_sale,
    house_second_hand_unlisted,
    residential,
    residential_aggregate,
);
