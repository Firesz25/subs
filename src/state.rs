use sea_orm::DbConn;
use tera::Tera;
#[derive(Debug, Clone)]
pub struct AppState {
    pub conn: DbConn,
    pub tera: Tera,
}

impl AppState {
    pub fn new(conn: DbConn) -> Self {
        let mut tera = Tera::new(concat!(env!("CARGO_MANIFEST_DIR"), "/templates/**/*")).unwrap();
        tera.autoescape_on(vec![".html.tera"]);
        Self { conn, tera }
    }
}
