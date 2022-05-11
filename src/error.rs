use actix_web::{http::header::ContentType, http::StatusCode, HttpResponse, ResponseError};
use once_cell::sync::Lazy;
use tera::{Tera, Context};
use thiserror::Error;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Error, Debug, Clone)]
pub enum Error {
    #[error("Page Not Found")]
    NotFound,
    #[error("Bad Request: {0}")]
    BadRequest(String),
    #[error("Internal Server Error")]
    InternalServerError,
}
static TERA: Lazy<Tera> = Lazy::new(|| {
    let mut tera = Tera::new(concat!(env!("CARGO_MANIFEST_DIR"), "/templates/**/*")).unwrap();
    tera.autoescape_on(vec![".html.tera", ".tera"]);
    tera
});
impl ResponseError for Error {
    fn status_code(&self) -> actix_web::http::StatusCode {
        match self {
            Error::NotFound => StatusCode::NOT_FOUND,
            Error::BadRequest(_) => StatusCode::BAD_REQUEST,
            Error::InternalServerError => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }

    fn error_response(&self) -> HttpResponse {
        let mut ctx = Context::new();
        ctx.insert("status_code", &self.status_code().as_str());
        ctx.insert("message", &self.to_string());
        let html = TERA.render("error/status.html.tera", &ctx).unwrap();
        HttpResponse::build(self.status_code()).content_type(ContentType::html()).body(html)
    }
}
