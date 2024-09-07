use std::sync::Arc;

use config::Config;

#[derive(Debug, Clone)]
pub struct Server {
    pub port: i64,
}

#[derive(Debug, Clone)]
pub struct Database {
    pub host: String,
    pub port: i64,
    pub user: String,
    pub password: String,
    pub dbname: String,
    pub schema: String,
}

#[derive(Debug, Clone)]
pub struct Setting {
    pub server: Server,
    pub database: Database,
}

impl Setting {
    pub fn new() -> Result<Arc<Setting>, config::ConfigError> {
        let settings = Config::builder()
            .add_source(config::File::with_name("Setting"))
            .build()
            .unwrap();

        Ok(Arc::new(Self {
            server: Server {
                port: settings.get_int("server.port").unwrap(),
            },
            database: Database {
                host: settings.get_string("database.host").unwrap(),
                port: settings.get_int("database.port").unwrap(),
                user: settings.get_string("database.user").unwrap(),
                password: settings.get_string("database.password").unwrap(),
                dbname: settings.get_string("database.dbname").unwrap(),
                schema: settings.get_string("database.schema").unwrap(),
            },
        }))
    }
}

impl Database {
    pub fn url_getting(&self) -> String {
        format!(
            "postgres://{}:{}@{}:{}/{}?options=-c search_path={}",
            self.user, self.password, self.host, self.port, self.dbname, self.schema,
        )
    }
}
