use crate::repository::schema::user_password_recovery;
use diesel::sql_types::Uuid;
use std::time::SystemTime;

// #[derive(Queryable, Identifiable)]
// pub struct UserPasswordRecovery {
//     pub id: Uuid,
//     pub user_id: i32,
//     pub issued_at: SystemTime,
// }

// #[derive(Insertable)]
// #[table_name = "user_password_recovery"]
// pub struct NewUserPasswordRecovery {
//     pub id: Uuid,
//     pub user_id: i32,
//     pub issued_at: SystemTime,
// }
