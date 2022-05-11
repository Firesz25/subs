use entity::prelude::*;
use sea_orm_migration::{
    prelude::*,
    sea_orm::{DbBackend, EntityTrait, Schema},
};

pub struct Migration;

impl MigrationName for Migration {
    fn name(&self) -> &str {
        "m20220101_000001_create_table"
    }
}

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager.create_table(create_users()).await.unwrap();
        manager.create_table(create_subs()).await.unwrap();
        manager
            .create_index(
                Index::create()
                    .table(Sub)
                    .col(SubColumn::Language)
                    .name("sub-lan")
                    .to_owned(),
            )
            .await
            .unwrap();
        manager
            .create_index(
                Index::create()
                    .table(Sub)
                    .col(SubColumn::Title)
                    .name("sub-tit")
                    .to_owned(),
            )
            .await
            .unwrap();
        manager
            .create_index(
                Index::create()
                    .table(Sub)
                    .col(SubColumn::CreateBy)
                    .name("sub-cby")
                    .to_owned(),
            )
            .await
            .unwrap();
        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let stmts = vec![seaorm_drop_stmt(User), seaorm_drop_stmt(Sub)];

        for stmt in stmts {
            manager.drop_table(stmt.to_owned()).await?;
        }

        manager
            .drop_index(Index::drop().table(Sub).to_owned())
            .await
            .unwrap();

        Ok(())
    }
}

#[allow(dead_code)]
fn seaorm_create_stmt(e: impl EntityTrait, dbd: DbBackend) -> TableCreateStatement {
    let schema = Schema::new(dbd);

    schema
        .create_table_from_entity(e)
        .if_not_exists()
        .to_owned()
}

fn create_users() -> TableCreateStatement {
    Table::create()
        .table(User)
        .if_not_exists()
        .col(
            ColumnDef::new(UserColumn::Id)
                .integer()
                .not_null()
                .auto_increment()
                .primary_key(),
        )
        .col(ColumnDef::new(UserColumn::Name).string().not_null())
        .col(
            ColumnDef::new(UserColumn::Email)
                .string()
                .not_null()
                .unique_key(),
        )
        .col(ColumnDef::new(UserColumn::Password).string().not_null())
        .col(ColumnDef::new(UserColumn::Image).string())
        .col(ColumnDef::new(UserColumn::Permision).string().not_null())
        .col(
            ColumnDef::new(UserColumn::UpdateAt)
                .timestamp_with_time_zone()
                .not_null(),
        )
        .col(
            ColumnDef::new(UserColumn::CreateAt)
                .timestamp_with_time_zone()
                .not_null(),
        )
        // .foreign_key(
        //     ForeignKey::create()
        //         .from(User, UserColumn::Id)
        //         .to(Sub, SubColumn::CreateBy)
        //         .name("fk-user-sub"),
        // )
        .to_owned()
}

fn create_subs() -> TableCreateStatement {
    Table::create()
        .table(Sub)
        .if_not_exists()
        .col(
            ColumnDef::new(SubColumn::Id)
                .integer()
                .not_null()
                .auto_increment()
                .primary_key(),
        )
        .col(ColumnDef::new(SubColumn::Path).string().not_null())
        .col(ColumnDef::new(SubColumn::Language).string())
        .col(ColumnDef::new(SubColumn::Title).string().not_null())
        .col(ColumnDef::new(SubColumn::CreateBy).integer().not_null())
        .col(ColumnDef::new(SubColumn::Description).string().not_null())
        .col(
            ColumnDef::new(SubColumn::UpdateAt)
                .timestamp_with_time_zone()
                .not_null(),
        )
        .col(
            ColumnDef::new(SubColumn::CreateAt)
                .timestamp_with_time_zone()
                .not_null(),
        )
        // .index(
        //     Index::create()
        //         .table(Sub)
        //         .col(SubColumn::Language)
        //         .name("sub-lan"),
        // )
        // .index(
        //     Index::create()
        //         .table(Sub)
        //         .col(SubColumn::Title)
        //         .name("sub-tit"),
        // )
        // .index(
        //     Index::create()
        //         .table(Sub)
        //         .col(SubColumn::CreateBy)
        //         .name("sub-cby"),
        // )
        .foreign_key(
            ForeignKey::create()
                .to(User, UserColumn::Id)
                .from(Sub, SubColumn::CreateBy)
                .name("fk-user-sub"),
        )
        .to_owned()
}

fn seaorm_drop_stmt(e: impl EntityTrait) -> TableDropStatement {
    Table::drop().table(e).if_exists().to_owned()
}
