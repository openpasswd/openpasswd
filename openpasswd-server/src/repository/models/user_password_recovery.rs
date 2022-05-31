use chrono::NaiveDateTime;

pub struct NewUserPasswordRecovery {
    pub user_id: i32,
    pub token: String,
    pub issued_at: NaiveDateTime,
    pub valid: bool,
}
