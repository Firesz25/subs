use actix_web::{HttpResponse, web, HttpRequest};
use tera::Context;
use crate::state::AppState;

pub async fn e404(data: web::Data<AppState>, req: HttpRequest) -> HttpResponse {
    let path = req.path();
    let tera = &data.tera;
    let mut ctx = Context::new();
    ctx.insert("title", "Page Not Found");
    ctx.insert("status_code", &404);
    ctx.insert("message", format!("Page {} Not Found", path).as_str());
    let html = tera.render("error/status.html.tera", &ctx).unwrap();
    HttpResponse::NotFound().body(html)
}


