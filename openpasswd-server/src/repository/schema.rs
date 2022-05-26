table! {
    account_groups (id) {
        id -> Int4,
        user_id -> Int4,
        name -> Varchar,
    }
}

table! {
    account_passwords (id) {
        id -> Int4,
        account_id -> Int4,
        username -> Varchar,
        password -> Bytea,
        created_date -> Timestamp,
    }
}

table! {
    accounts (id) {
        id -> Int4,
        user_id -> Int4,
        account_groups_id -> Int4,
        level -> Int2,
        name -> Varchar,
    }
}

table! {
    devices (id) {
        id -> Int4,
        user_id -> Int4,
        name -> Varchar,
        last_access -> Timestamp,
        active -> Bool,
        public_key -> Text,
    }
}

table! {
    user_password_recovery (id) {
        id -> Uuid,
        user_id -> Int4,
        issued_at -> Nullable<Timestamp>,
    }
}

table! {
    users (id) {
        id -> Int4,
        name -> Varchar,
        email -> Varchar,
        password -> Text,
        master_key -> Nullable<Varchar>,
        last_login -> Nullable<Timestamp>,
        fail_attempts -> Int2,
        last_attempt -> Nullable<Timestamp>,
    }
}

joinable!(account_groups -> users (user_id));
joinable!(account_passwords -> accounts (account_id));
joinable!(accounts -> account_groups (account_groups_id));
joinable!(accounts -> users (user_id));
joinable!(devices -> users (user_id));
joinable!(user_password_recovery -> users (user_id));

allow_tables_to_appear_in_same_query!(
    account_groups,
    account_passwords,
    accounts,
    devices,
    user_password_recovery,
    users,
);
