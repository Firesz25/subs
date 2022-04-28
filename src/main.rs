mod config;
mod handlers;
mod log;
mod model;
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
    migration::Migrator::down(&conn, None).await.unwrap();
    let state = web::Data::new(state::AppState::new(conn));
    HttpServer::new(move || {
        App::new()
            .app_data(state.clone())
            .wrap(middleware::Logger::default())
            .configure(handlers::route_file)
    })
    .bind(CFG.srv_url())
    .unwrap()
    .run()
    .await
    .unwrap();
}
