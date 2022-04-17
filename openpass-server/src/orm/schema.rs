table! {
    accounts (id) {
        id -> Int4,
        level -> Int4,
        name -> Varchar,
        username -> Varchar,
        password -> Text,
        user_id -> Int4,
    }
}

table! {
    devices (id) {
        id -> Int4,
        name -> Varchar,
        last_access -> Timestamp,
        active -> Bool,
        public_key -> Text,
        user_id -> Int4,
    }
}

table! {
    users (id) {
        id -> Int4,
        name -> Varchar,
        email -> Varchar,
        password -> Text,
    }
}

joinable!(accounts -> users (user_id));
joinable!(devices -> users (user_id));

allow_tables_to_appear_in_same_query!(
    accounts,
    devices,
    users,
);
