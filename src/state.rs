use sea_orm::DbConn;
#[derive(Debug, Clone)]
pub struct AppState {
    pub conn: DbConn,
}

impl AppState {
    pub fn new(conn: DbConn) -> Self {
        Self { conn }
    }
}
