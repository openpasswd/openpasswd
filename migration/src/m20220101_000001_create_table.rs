use sea_orm_migration::{prelude::*, sea_orm::EntityTrait};

pub struct Migration;

impl MigrationName for Migration {
    fn name(&self) -> &str {
        "m20220101_000001_create_table"
    }
}

fn stmt_users() -> TableCreateStatement {
    sea_query::Table::create()
        .table(entity::users::Entity)
        .if_not_exists()
        .col(
            ColumnDef::new(entity::users::Column::Id)
                .integer()
                .not_null()
                .auto_increment()
                .primary_key(),
        )
        .col(
            ColumnDef::new(entity::users::Column::Name)
                .string_len(100)
                .not_null(),
        )
        .col(
            ColumnDef::new(entity::users::Column::Email)
                .string_len(100)
                .not_null()
                .unique_key(),
        )
        .col(
            ColumnDef::new(entity::users::Column::Password)
                .string()
                .not_null(),
        )
        .col(ColumnDef::new(entity::users::Column::MasterKey).string_len(32))
        .col(ColumnDef::new(entity::users::Column::LastLogin).timestamp())
        .col(
            ColumnDef::new(entity::users::Column::FailAttempts)
                .small_integer()
                .default(0)
                .not_null(),
        )
        .col(ColumnDef::new(entity::users::Column::LastAttempt).timestamp())
        .to_owned()
}

fn stmt_user_password_recovery() -> TableCreateStatement {
    sea_query::Table::create()
        .table(entity::user_password_recovery::Entity)
        .if_not_exists()
        .col(
            ColumnDef::new(entity::user_password_recovery::Column::Id)
                .uuid()
                .not_null()
                .primary_key(),
        )
        .col(
            ColumnDef::new(entity::user_password_recovery::Column::UserId)
                .integer()
                .not_null(),
        )
        .col(
            ColumnDef::new(entity::user_password_recovery::Column::IssuedAt)
                .date_time()
                .not_null(),
        )
        .foreign_key(
            ForeignKey::create()
                .from(
                    entity::user_password_recovery::Entity,
                    entity::user_password_recovery::Column::UserId,
                )
                .to(entity::users::Entity, entity::users::Column::Id)
                .on_delete(ForeignKeyAction::NoAction)
                .on_update(ForeignKeyAction::NoAction),
        )
        .to_owned()
}

fn stmt_account_groups() -> TableCreateStatement {
    sea_query::Table::create()
        .table(entity::account_groups::Entity)
        .if_not_exists()
        .col(
            ColumnDef::new(entity::account_groups::Column::Id)
                .integer()
                .not_null()
                .auto_increment()
                .primary_key(),
        )
        .col(
            ColumnDef::new(entity::account_groups::Column::UserId)
                .integer()
                .not_null(),
        )
        .col(
            ColumnDef::new(entity::account_groups::Column::Name)
                .string_len(50)
                .not_null(),
        )
        .foreign_key(
            ForeignKey::create()
                .from(
                    entity::account_groups::Entity,
                    entity::account_groups::Column::UserId,
                )
                .to(entity::users::Entity, entity::users::Column::Id)
                .on_delete(ForeignKeyAction::NoAction)
                .on_update(ForeignKeyAction::NoAction),
        )
        .to_owned()
}

fn stmt_accounts() -> TableCreateStatement {
    sea_query::Table::create()
        .table(entity::accounts::Entity)
        .if_not_exists()
        .col(
            ColumnDef::new(entity::accounts::Column::Id)
                .integer()
                .not_null()
                .auto_increment()
                .primary_key(),
        )
        .col(
            ColumnDef::new(entity::accounts::Column::UserId)
                .integer()
                .not_null(),
        )
        .col(
            ColumnDef::new(entity::accounts::Column::AccountGroupsId)
                .integer()
                .not_null(),
        )
        .col(ColumnDef::new(entity::accounts::Column::Level).small_integer())
        .col(
            ColumnDef::new(entity::accounts::Column::Name)
                .string_len(50)
                .not_null(),
        )
        .foreign_key(
            ForeignKey::create()
                .from(entity::accounts::Entity, entity::accounts::Column::UserId)
                .to(entity::users::Entity, entity::users::Column::Id)
                .on_delete(ForeignKeyAction::NoAction)
                .on_update(ForeignKeyAction::NoAction),
        )
        .foreign_key(
            ForeignKey::create()
                .from(
                    entity::accounts::Entity,
                    entity::accounts::Column::AccountGroupsId,
                )
                .to(
                    entity::account_groups::Entity,
                    entity::account_groups::Column::Id,
                )
                .on_delete(ForeignKeyAction::NoAction)
                .on_update(ForeignKeyAction::NoAction),
        )
        .to_owned()
}

fn stmt_account_passwords() -> TableCreateStatement {
    sea_query::Table::create()
        .table(entity::account_passwords::Entity)
        .if_not_exists()
        .col(
            ColumnDef::new(entity::account_passwords::Column::Id)
                .integer()
                .not_null()
                .auto_increment()
                .primary_key(),
        )
        .col(
            ColumnDef::new(entity::account_passwords::Column::AccountId)
                .integer()
                .not_null(),
        )
        .col(
            ColumnDef::new(entity::account_passwords::Column::Username)
                .string_len(100)
                .not_null(),
        )
        .col(
            ColumnDef::new(entity::account_passwords::Column::Password)
                .binary()
                .not_null(),
        )
        .col(
            ColumnDef::new(entity::account_passwords::Column::CreatedDate)
                .date_time()
                .not_null(),
        )
        .foreign_key(
            ForeignKey::create()
                .from(
                    entity::account_passwords::Entity,
                    entity::account_passwords::Column::AccountId,
                )
                .to(entity::accounts::Entity, entity::accounts::Column::Id)
                .on_delete(ForeignKeyAction::NoAction)
                .on_update(ForeignKeyAction::NoAction),
        )
        .to_owned()
}

fn stmt_devices() -> TableCreateStatement {
    sea_query::Table::create()
        .table(entity::devices::Entity)
        .if_not_exists()
        .col(
            ColumnDef::new(entity::devices::Column::Id)
                .uuid()
                .not_null()
                .primary_key(),
        )
        .col(
            ColumnDef::new(entity::devices::Column::UserId)
                .integer()
                .not_null(),
        )
        .col(
            ColumnDef::new(entity::devices::Column::Name)
                .string_len(100)
                .not_null(),
        )
        .col(
            ColumnDef::new(entity::devices::Column::LastAccess)
                .date_time()
                .not_null(),
        )
        .col(
            ColumnDef::new(entity::devices::Column::Active)
                .boolean()
                .not_null(),
        )
        .col(
            ColumnDef::new(entity::devices::Column::PublicKey)
                .string()
                .not_null(),
        )
        .foreign_key(
            ForeignKey::create()
                .from(entity::devices::Entity, entity::devices::Column::UserId)
                .to(entity::users::Entity, entity::users::Column::Id)
                .on_delete(ForeignKeyAction::NoAction)
                .on_update(ForeignKeyAction::NoAction),
        )
        .index(
            Index::create()
                .col(entity::devices::Column::Id)
                .col(entity::devices::Column::Name)
                .unique(),
        )
        .to_owned()
}

fn drop_stmt<E: EntityTrait>(e: E) -> TableDropStatement {
    Table::drop().table(e).if_exists().to_owned()
}

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager.create_table(stmt_users()).await?;
        manager.create_table(stmt_user_password_recovery()).await?;
        manager.create_table(stmt_account_groups()).await?;
        manager.create_table(stmt_accounts()).await?;
        manager.create_table(stmt_account_passwords()).await?;
        manager.create_table(stmt_devices()).await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(drop_stmt(entity::devices::Entity))
            .await?;
        manager
            .drop_table(drop_stmt(entity::account_passwords::Entity))
            .await?;
        manager
            .drop_table(drop_stmt(entity::accounts::Entity))
            .await?;
        manager
            .drop_table(drop_stmt(entity::account_groups::Entity))
            .await?;
        manager
            .drop_table(drop_stmt(entity::user_password_recovery::Entity))
            .await?;
        manager.drop_table(drop_stmt(entity::users::Entity)).await?;

        Ok(())
    }
}
