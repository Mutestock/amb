table! {
    images (id) {
        id -> Int4,
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

allow_tables_to_appear_in_same_query!(
    images,
    users,
);
