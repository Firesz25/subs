use crate::state::AppState;
use actix_web::{web, HttpResponse, Responder};
use entity::prelude::*;
use sea_orm::{EntityTrait, PaginatorTrait, QueryOrder};
use serde::Deserialize;

pub fn route_sub(cfg: &mut web::ServiceConfig) {
    cfg.route("/sub", web::get().to(list));
    cfg.route("/sub/search", web::get().to(search));
    cfg.route("/sub/{id}", web::get().to(get));
}

#[derive(Debug, Deserialize)]
struct Params {
    page: Option<usize>,
    subs_per_page: Option<usize>,
}

async fn list(state: web::Data<AppState>, query: web::Query<Params>) -> HttpResponse {
    let conn = &state.conn;
    let query = query.into_inner();
    let page = query.page.unwrap_or(1);

    let subs_per_page = query.subs_per_page.unwrap_or(10);
    let paginator = Sub::find()
        .order_by_asc(SubColumn::CreateAt)
        .paginate(conn, subs_per_page);
    let subs = paginator.fetch_page(page - 1).await.ok().unwrap();
    HttpResponse::Ok().json(subs)
}

async fn get(state: web::Data<AppState>, id: web::Path<i32>) -> impl Responder {
    let conn = &state.conn;
    let sub = Sub::find_by_id(id.into_inner()).one(conn).await.unwrap();
    match sub {
        Some(sub) => HttpResponse::Ok().json(sub),
        None => HttpResponse::NotFound().finish(),
    }
}

async fn search(state: web::Data<AppState>, form: web::Form<Search>) -> HttpResponse {
    let conn = &state.conn;
    let form = form.into_inner();
    let sub = match form.language {
        Language::All => Sub::find_by_title(&form.title).one(conn).await.unwrap(),
        Language::Orginal => Sub::find_by_orginal(&form.title).one(conn).await.unwrap(),
        Language::English => Sub::find_by_english(&form.title).one(conn).await.unwrap(),
    };
    match sub {
        Some(sub) => HttpResponse::Ok().json(sub),
        None => HttpResponse::NotFound().finish(),
    }
}

#[derive(Debug, Deserialize)]
struct Search {
    #[serde(default)]
    pub language: Language,
    pub title: String,
}

impl std::str::FromStr for Language {
    type Err = Box<dyn std::error::Error>;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "orginal" => Ok(Language::Orginal),
            "english" => Ok(Language::English),
            _ => Ok(Language::All),
        }
    }
}
#[derive(Debug, Deserialize)]
enum Language {
    All,
    Orginal,
    English,
}

impl Default for Language {
    fn default() -> Self {
        Language::All
    }
}
