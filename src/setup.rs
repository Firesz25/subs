use crate::config::CFG;
use entity::prelude::{Sub, SubActiveModel, User, UserActiveModel};
use sea_orm::{ActiveModelTrait, DbConn, Set};
pub async fn database(conn: &DbConn) {
    let user = User::find_by_email(&CFG.root.email.clone())
        .one(conn)
        .await
        .unwrap();
    if user.is_none() {
        UserActiveModel {
            name: Set(CFG.root.name.clone()),
            email: Set(CFG.root.email.clone()),
            permision: Set("".to_string()),
            password: Set(CFG.root.password.clone()),
            ..Default::default()
        }
        .save(conn)
        .await
        .unwrap();
    }
    let sub = Sub::find_by_orginal("title").one(conn).await.unwrap();
    if sub.is_none() {
        SubActiveModel {
            title: Set("title".to_string()),
            language: Set(Some("language".to_string())),
            create_by: Set(1),
            path: Set("path".to_string()),
            description: Set("description".to_string()),
            ..Default::default()
        }
        .insert(conn)
        .await
        .unwrap();
    }
}