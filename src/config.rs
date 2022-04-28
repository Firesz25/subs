use once_cell::sync::Lazy;
use serde::Deserialize;
use std::fs;

pub static CFG: Lazy<Config> = Lazy::new(|| Config::new());

#[derive(Debug, Clone, Deserialize)]
pub struct Config {
    db: Database,
    srv: Server,
    pub root: User,
    pub secret: String,
}

impl Config {
    pub fn new() -> Self {
        ron::from_str(&fs::read_to_string("Config.ron").unwrap()).unwrap()
    }

    pub fn db_url(&self) -> String {
        if self.db.protocol == "sqlite" {
            fs::File::create(format!("{}.db", self.db.database)).unwrap();
            format!("sqlite://{}.db", self.db.database)
        } else if self.db.port.is_some() {
            format!(
                "{}://{}:{}@{}:{}/{}",
                self.db.protocol,
                self.db.user,
                self.db.password,
                self.db.host,
                self.db.port.unwrap(),
                self.db.database
            )
        } else {
            format!(
                "{}://{}:{}@{}/{}",
                self.db.protocol, self.db.user, self.db.password, self.db.host, self.db.database
            )
        }
    }

    pub fn srv_url(&self) -> String {
        format!("{}:{}", self.srv.host, self.srv.port)
    }
}

#[derive(Debug, Clone, Default, Deserialize)]
struct Database {
    protocol: String,
    user: String,
    password: String,
    host: String,
    port: Option<u16>,
    database: String,
}

#[derive(Debug, Clone, Deserialize)]
struct Server {
    host: String,
    port: u16,
}

#[derive(Debug, Clone, Deserialize)]
pub struct User {
    pub name: String,
    pub email: String,
    pub password: String,
}
