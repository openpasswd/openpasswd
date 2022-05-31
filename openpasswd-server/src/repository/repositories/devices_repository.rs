use crate::repository::Repository;
use async_trait::async_trait;
use sea_orm::{ColumnTrait, EntityTrait, QueryFilter};

#[async_trait]
pub trait DevicesRepository {
    async fn devices_find_device_name(&self, user_id: i32, device_name: &str) -> Option<String>;
}
#[async_trait]
impl DevicesRepository for Repository {
    async fn devices_find_device_name(&self, user_id: i32, device_name: &str) -> Option<String> {
        match entity::devices::Entity::find()
            .filter(
                entity::devices::Column::UserId
                    .eq(user_id)
                    .and(entity::devices::Column::Name.eq(device_name)),
            )
            .one(&self.db)
            .await
            .unwrap()
        {
            Some(result) => Some(result.name.clone()),
            None => None,
        }
    }
}
