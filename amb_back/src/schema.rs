table! {
    images (id) {
        id -> Int4,
        user_id -> Int4,
        title -> Text,
        path -> Text,
        description -> Nullable<Text>,
        created_at -> Nullable<Timestamp>,
    }
}

table! {
    users (id) {
        id -> Int4,
        username -> Text,
        password -> Varchar,
        email -> Text,
        description -> Nullable<Varchar>,
        created_at -> Nullable<Timestamp>,
        updated_at -> Nullable<Timestamp>,
        last_login -> Nullable<Timestamp>,
        admin -> Bool,
        salt -> Nullable<Varchar>,
    }
}

table! {
    tracks (id) {
        id -> Int4,
        user_id -> Int4,
        title -> Varchar,
        uuid_fname -> Varchar,
        path -> Varchar,
        description -> Nullable<Varchar>,
        uploaded_at -> Nullable<Timestamp>,
        duration -> Nullable<Int4>,
        credits -> Varchar,
    }
}

joinable!(images -> users (user_id));
joinable!(tracks -> users (user_id));

allow_tables_to_appear_in_same_query!(images, users, tracks);
