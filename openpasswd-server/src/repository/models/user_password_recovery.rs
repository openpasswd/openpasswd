use std::time::SystemTime;

use uuid::Uuid;

pub struct UserPasswordRecovery {
    pub id: Uuid,
    pub user_id: i32,
    pub issued_at: SystemTime,
}

// #[derive(Insertable)]
// #[table_name = "user_password_recovery"]
// pub struct NewUserPasswordRecovery {
//     pub id: Uuid,
//     pub user_id: i32,
//     pub issued_at: SystemTime,
// }
