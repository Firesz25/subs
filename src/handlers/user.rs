use actix_web::{HttpResponse, web};
use entity::prelude::*;
use sea_orm::{ActiveModelTrait, Set};
use serde::Deserialize;

use crate::state::AppState;

pub fn route_user(cfg: &mut web::ServiceConfig) {
    cfg.route("/sign_up", web::post().to(sign_up));
}

async fn sign_up(state: web::Data<AppState>, form: web::Form<User>) -> HttpResponse {
    let conn = &state.conn;
    let form = form.into_inner();
    UserActiveModel {
        name: Set(form.name),
        email: Set(form.email),
        password: Set(form.password),
        ..Default::default()
    }.insert(conn).await.unwrap();
    HttpResponse::Ok().json(())
}  

#[derive(Debug, Deserialize)]
struct User {
    name: String,
    email: String,
    password: String,
}
    