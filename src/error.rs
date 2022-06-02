use actix_web::{HttpResponse, HttpRequest, ResponseError, http::StatusCode, web};
use tera::Context;
use crate::state::AppState;
use sea_orm::DbErr;
use thiserror::Error;
use std::error::Error as StdError;
use tracing::error;

pub type Result<T> = std::result::Result<T, Error>;

pub async fn e404(req: HttpRequest, state: web::Data<AppState>) -> HttpResponse {
    e(state.get_ref(), 404, "Page Not Found", &format!("Page {} Not Found", req.path()))
}

pub fn e(state: &AppState, status_code: u16, title: &str, message: &str) -> HttpResponse {
    let tera = &state.tera;
    let mut ctx = Context::new();
    ctx.insert("title", title);
    ctx.insert("status_code", &status_code);
    ctx.insert("message", message);
    let html = tera.render("error/status.html.tera", &ctx).unwrap();
    HttpResponse::build(StatusCode::from_u16(status_code).unwrap()).body(html)
}

#[derive(Debug, Error)]
pub enum Error {
    #[error("Database {0}")]
    DB(#[from] #[source] DbErr),
    #[error("Tera: {0}")]
    Tera(#[from] #[source] tera::Error),
    #[error("IO: {0}")]
    IO(#[from] #[source] std::io::Error),
    #[error("Custom: {0}")]
    Custom(#[from] #[source] Box<dyn StdError + Send + Sync>)
}

impl ResponseError for Error {
    fn error_response(&self) -> HttpResponse {
        error!("{}", self.to_string()); 
        HttpResponse::build(self.status_code()).body(())
    }
}

#[test]
fn test_error() {
    let err = Error::DB(DbErr::Custom("test".to_string()));
    assert_eq!(err.status_code(), 500);
    assert_eq!(err.to_string(), "Database Custom Error: test".to_string());
}
