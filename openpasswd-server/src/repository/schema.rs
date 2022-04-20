table! {
    account_groups (id) {
        id -> Int4,
        name -> Varchar,
        user_id -> Int4,
    }
}

table! {
    accounts (id) {
        id -> Int4,
        level -> Int2,
        name -> Varchar,
        username -> Varchar,
        password -> Text,
        account_groups_id -> Int4,
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
        last_login -> Nullable<Timestamp>,
        fail_attempts -> Int2,
        last_attempt -> Nullable<Timestamp>,
    }
}

joinable!(account_groups -> users (user_id));
joinable!(accounts -> account_groups (account_groups_id));
joinable!(accounts -> users (user_id));
joinable!(devices -> users (user_id));

allow_tables_to_appear_in_same_query!(
    account_groups,
    accounts,
    devices,
    users,
);
