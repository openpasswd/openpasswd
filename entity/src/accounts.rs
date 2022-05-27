//! SeaORM Entity. Generated by sea-orm-codegen 0.8.0

use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "accounts")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    pub user_id: i32,
    pub account_groups_id: i32,
    pub level: i16,
    pub name: String,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::account_groups::Entity",
        from = "Column::AccountGroupsId",
        to = "super::account_groups::Column::Id",
        on_update = "NoAction",
        on_delete = "NoAction"
    )]
    AccountGroups,
    #[sea_orm(
        belongs_to = "super::users::Entity",
        from = "Column::UserId",
        to = "super::users::Column::Id",
        on_update = "NoAction",
        on_delete = "NoAction"
    )]
    Users,
    #[sea_orm(has_many = "super::account_passwords::Entity")]
    AccountPasswords,
}

impl Related<super::account_groups::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::AccountGroups.def()
    }
}

impl Related<super::users::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Users.def()
    }
}

impl Related<super::account_passwords::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::AccountPasswords.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
