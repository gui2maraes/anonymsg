use std::str::FromStr;

use config::{Config, FileFormat};
use secrecy::{ExposeSecret, Secret};
use serde::Deserialize;
use sqlx::postgres::{PgConnectOptions, PgSslMode};
use std::sync::LazyLock;

static ENVIRONMENT: LazyLock<Environment> = LazyLock::new(|| {
    std::env::var("APP_ENVIRONMENT")
        .unwrap_or_else(|_| "local".into())
        .parse()
        .expect("Failed to parse APP_ENVIRONMENT")
});
pub fn get_environment() -> Environment {
    *ENVIRONMENT
}

pub fn get_configuration() -> Result<Settings, config::ConfigError> {
    let environment = get_environment();
    let environment_config = config::File::new(
        &format!("config/{}", environment.as_str()),
        FileFormat::Yaml,
    )
    .required(true);
    let settings = Config::builder().add_source(environment_config).build()?;

    settings.try_deserialize()
}
#[derive(Deserialize)]
pub struct Settings {
    pub database: DatabaseSettings,
    pub application: ApplicationSettings,
}
#[derive(Deserialize)]
pub struct ApplicationSettings {
    pub host: String,
    pub port: u16,
}

#[derive(Deserialize)]
pub struct DatabaseSettings {
    pub username: String,
    pub password: Secret<String>,
    pub host: String,
    pub database_name: String,
    pub port: u16,
    pub require_ssl: bool,
}

impl DatabaseSettings {
    pub fn connection_string(&self) -> Secret<String> {
        Secret::new(format!(
            "postgres://{}:{}@{}:{}/{}",
            self.username,
            self.password.expose_secret(),
            self.host,
            self.port,
            self.database_name
        ))
    }
    pub fn connect_options(&self) -> PgConnectOptions {
        let ssl_mode = if self.require_ssl {
            PgSslMode::Require
        } else {
            PgSslMode::Prefer
        };
        PgConnectOptions::new()
            .ssl_mode(ssl_mode)
            .username(&self.username)
            .database(&self.database_name)
            .password(self.password.expose_secret())
            .port(self.port)
            .host(&self.host)
    }
}
#[derive(Debug, Clone, Copy)]
pub enum Environment {
    Local,
    Production,
}
impl Environment {
    pub fn as_str(&self) -> &'static str {
        match *self {
            Self::Local => "local",
            Self::Production => "production",
        }
    }
    pub fn is_production(self) -> bool {
        matches!(self, Environment::Production)
    }
    pub fn is_local(self) -> bool {
        matches!(self, Environment::Local)
    }
}
impl FromStr for Environment {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            "local" => Self::Local,
            "production" => Self::Production,
            s => {
                return Err(format!(
                    "`{s}` is not a supported environment. Use either `local` or `production`."
                ))
            }
        })
    }
}
