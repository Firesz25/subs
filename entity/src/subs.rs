use crate::prelude::*;
use chrono::Utc;
use sea_orm::{entity::prelude::*, Condition, Set};
use serde::Serialize;

#[derive(Debug, Clone, PartialEq, DeriveEntityModel, Serialize)]
#[sea_orm(table_name = "subs")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    pub path: String,
    #[sea_orm(indexed, nullable)]
    pub language: Option<String>,
    #[sea_orm(indexed)]
    pub title: String,
    #[sea_orm(indexed)]
    pub create_by: i32,
    pub description: String,
    pub create_at: DateTimeWithTimeZone,
    pub update_at: DateTimeWithTimeZone,
}

#[derive(Debug, Clone, Copy, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(belongs_to = "User", from = "Column::CreateBy", to = "UserColumn::Id")]
    User,
}

impl Entity {
    pub fn find_by_id(id: i32) -> Select<Self> {
        Self::find().filter(Column::Id.eq(id))
    }
    pub fn find_by_title(title: &str) -> Select<Self> {
        Self::find().filter(
            Condition::any()
                .add(Column::Language.contains(title))
                .add(Column::Title.contains(title)),
        )
    }

    pub fn find_by_english(english: &str) -> Select<Self> {
        Self::find().filter(Column::Language.contains(english))
    }

    pub fn find_by_orginal(orginal: &str) -> Select<Self> {
        Self::find().filter(Column::Title.contains(orginal))
    }

    pub fn find_by_user_id(user_id: i32) -> Select<Self> {
        Self::find().filter(Column::CreateBy.eq(user_id))
    }

    pub fn find_by_date(date: &str) -> Select<Self> {
        Self::find().filter(Column::CreateAt.eq(date))
    }
}

impl Related<User> for Entity {
    fn to() -> RelationDef {
        Relation::User.def()
    }
}

impl ActiveModelBehavior for ActiveModel {
    fn new() -> Self {
        Self {
            create_at: Set(Utc::now().into()),
            update_at: Set(Utc::now().into()),
            ..ActiveModelTrait::default()
        }
    }
}
