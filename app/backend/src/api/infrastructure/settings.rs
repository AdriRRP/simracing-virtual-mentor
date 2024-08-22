use config::{Config, ConfigError, Environment, File};
use serde::{Deserialize, Serialize};
use std::fmt::{Display, Formatter};

const BACKEND_CONFIG_PATH_ENV: &str = "BACKEND_CONFIG_PATH";
const BACKEND_DEFAULT_CONFIG_FILE_PATH: &str = "backend-config";
#[derive(Debug, Deserialize)]
pub struct Settings {
    pub server: Server,
    pub event_bus: EventBus,
    pub log_level: LogLevel,
    pub mongo: Mongo,
    pub fcm_grid: FcmGrid,
}

#[derive(Debug, Deserialize)]
pub struct Server {
    pub host: String,
    pub port: u16,
}

#[derive(Debug, Deserialize)]
pub struct EventBus {
    pub capacity: usize,
}
#[derive(Debug, Serialize, Deserialize)]
pub enum LogLevel {
    Trace,
    Debug,
    Info,
    Warn,
    Error,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Mongo {
    pub uri: String,
    pub database: String,
    pub collections: MongoCollections,
    pub user: String,
    pub pass: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MongoCollections {
    pub analysis: String,
    pub file: String,
    pub lap: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FcmGrid {
    pub c: InitMaxMinUsize,
    pub m: InitMaxMinF64,
    pub max_iter: InitMaxMinUsize,
    pub error: InitMaxMinF64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct InitMaxMinUsize {
    pub init: usize,
    pub max: Option<usize>,
    pub inc: Option<usize>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct InitMaxMinF64 {
    pub init: f64,
    pub max: Option<f64>,
    pub inc: Option<f64>,
}



impl Display for LogLevel {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Trace => write!(f, "trace"),
            Self::Debug => write!(f, "debug"),
            Self::Info => write!(f, "info"),
            Self::Warn => write!(f, "warn"),
            Self::Error => write!(f, "error"),
        }
    }
}

impl Settings {
    /// # Errors
    ///
    /// Will return `Err` if it is not possible to build or deserialize the configuration file.
    pub fn load() -> Result<Self, ConfigError> {
        let config_path = std::env::var(BACKEND_CONFIG_PATH_ENV)
            .unwrap_or_else(|_| String::from(BACKEND_DEFAULT_CONFIG_FILE_PATH));

        let config = Config::builder()
            .add_source(File::with_name(config_path.as_str()))
            // Allow environment variables to set/override config parsing '__' as '.'
            // Keep '_' is needed due to attribute names
            .add_source(Environment::with_prefix("BACKEND").separator("__"))
            .build()?;

        config.try_deserialize()
    }
}
