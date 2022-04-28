use crate::prelude::*;
use chrono::Utc;
use sea_orm::{entity::prelude::*, Set};
use serde::Serialize;

#[derive(Debug, Clone, PartialEq, DeriveEntityModel, Serialize)]
#[sea_orm(table_name = "users")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    pub name: String,
    #[sea_orm(unique)]
    pub email: String,
    #[serde(skip_serializing)]
    pub password: String,
    #[sea_orm(nullable)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image: Option<String>,
    #[sea_orm(default_value = "0")]
    pub permision: String,
    pub create_at: DateTimeWithTimeZone,
    pub update_at: DateTimeWithTimeZone,
}

#[derive(Debug, Clone, Copy, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(has_many = "Sub")]
    Sub,
}

impl Entity {
    pub fn find_by_email(email: &str) -> Select<Self> {
        Self::find().filter(Column::Email.eq(email))
    }
}

impl Related<Sub> for Entity {
    fn to() -> RelationDef {
        Relation::Sub.def()
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
