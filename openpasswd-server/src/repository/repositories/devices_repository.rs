use crate::repository::models::device::Device;
use crate::repository::schema::devices::dsl as devices_dsl;
use crate::repository::Repository;
use diesel::prelude::*;

pub trait DevicesRepository {
    fn devices_find_device_name(&self, user_id: i32, device_name: &str) -> Option<String>;
}
impl DevicesRepository for Repository {
    fn devices_find_device_name(&self, user_id: i32, device_name: &str) -> Option<String> {
        let connection = &self.pool.get().unwrap();
        match devices_dsl::devices
            .filter(
                devices_dsl::user_id
                    .eq(user_id)
                    .and(devices_dsl::name.eq(device_name)),
            )
            .load::<Device>(connection)
        {
            Ok(result) => {
                if let Some(next) = result.first() {
                    Some(next.name.clone())
                } else {
                    None
                }
            }
            Err(e) => panic!("{e}"),
        }
    }
}
