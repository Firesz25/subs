mod config;
mod controler;
mod error;
mod handlers;
mod setup;
mod state;

use actix_web::{middleware, rt, web, App, HttpServer};
use config::CFG;

fn main() {
    let sys = rt::System::new();
    sys.block_on(real_main());
    let _ = sys.run();
}

async fn real_main() {
    setup::log();
    let conn = sea_orm::Database::connect(CFG.db_url()).await.unwrap();
    setup::database(&conn).await;
    let state = web::Data::new(state::AppState::new(conn));
    HttpServer::new(move || {
        App::new()
            .app_data(state.clone())
            .wrap(middleware::Logger::default())
            .configure(handlers::route_file)
            .configure(handlers::route_sub)
            .configure(handlers::route_user)
            .service(actix_files::Files::new("/static", "./static/").prefer_utf8(true))
            .default_service(web::get().to(error::e404))
    })
    .bind(CFG.srv_url())
    .unwrap()
    .run()
    .await
    .unwrap();
}
