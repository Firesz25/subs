mod config;
mod controler;
mod error;
mod handlers;
mod log;
mod setup;
mod state;

use actix_web::{middleware, rt, web, App, HttpServer};
use config::CFG;
use migration::MigratorTrait;

fn main() {
    let sys = rt::System::new();
    sys.block_on(real_main());
    let _ = sys.run();
}

async fn real_main() {
    log::setup();
    let conn = sea_orm::Database::connect(CFG.db_url()).await.unwrap();
    migration::Migrator::up(&conn, None).await.unwrap();
    setup::database(&conn).await;
    let state = web::Data::new(state::AppState::new(conn));
    HttpServer::new(move || {
        App::new()
            .app_data(state.clone())
            .wrap(error::error_handlers())
            .wrap(middleware::Logger::default())
            .configure(handlers::route_file)
            .configure(handlers::route_sub)
            .configure(handlers::route_user)
            // .default_service(web::get().to(e404))
            // .service(web::scope("").wrap(error::error_handlers()))
    })
    .bind(CFG.srv_url())
    .unwrap()
    .run()
    .await
    .unwrap();
}

// async fn e404(state: actix_web::web::Data<state::AppState>) -> actix_web::HttpResponse {
//     let tera = state.tera.clone();
//     let mut ctx = tera::Context::new();
//     ctx.insert("status_code", &404);
//     ctx.insert("message", "Page does not found");
//     let html = tera.render("error/status.html.tera", &ctx).unwrap();
//     actix_web::HttpResponse::NotFound()
//         .content_type(actix_web::http::header::ContentType::html())
//         .body(html)
// }
