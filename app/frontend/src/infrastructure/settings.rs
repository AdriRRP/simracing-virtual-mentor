use config::{Config, ConfigError, Environment, File as ConfigFile};
use serde::Deserialize;

const FRONTEND_CONFIG_PATH_ENV: &str = "FRONTEND_CONFIG_PATH";
const FRONTEND_DEFAULT_CONFIG_FILE_PATH: &str = "frontend-config";
#[derive(Debug, Deserialize)]
pub struct Settings {
    pub endpoints: Endpoints,
}

#[derive(Debug, Deserialize)]
pub struct Endpoints {
    pub analysis: Analysis,
    pub file: File,
    pub lap: Lap,
    pub ibt_extractor: IbtExtractor,
}

#[derive(Debug, Deserialize)]
pub struct Analysis {
    pub server: String,
    pub create: String,
    pub delete: String,
    pub find_by_id: String,
    pub find_by_criteria: String,
}

#[derive(Debug, Deserialize)]
pub struct File {
    pub server: String,
    pub delete: String,
    pub find_by_id: String,
    pub find_by_criteria: String,
}

#[derive(Debug, Deserialize)]
pub struct Lap {
    pub server: String,
    pub delete: String,
    pub find_by_id: String,
    pub find_by_criteria: String,
    pub find_header_by_id: String,
    pub find_header_by_criteria: String,
}

#[derive(Debug, Deserialize)]
pub struct IbtExtractor {
    pub server: String,
    pub upload: String,
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            endpoints: Endpoints {
                analysis: Analysis {
                    server: "http://127.0.0.1:16666".to_string(),
                    create: "/analysis/delete".to_string(),
                    delete: "/analysis/delete".to_string(),
                    find_by_id: "/analysis/find".to_string(),
                    find_by_criteria: "/analysis/find".to_string(),
                },
                file: File {
                    server: "http://127.0.0.1:16666".to_string(),
                    delete: "/file/delete".to_string(),
                    find_by_id: "/file/find".to_string(),
                    find_by_criteria: "/file/find".to_string(),
                },
                lap: Lap {
                    server: "http://127.0.0.1:16666".to_string(),
                    delete: "/lap/delete".to_string(),
                    find_by_id: "/lap/find".to_string(),
                    find_by_criteria: "/lap/find".to_string(),
                    find_header_by_id: "/lap/find/header".to_string(),
                    find_header_by_criteria: "/lap/find/header".to_string(),
                },
                ibt_extractor: IbtExtractor {
                    server: "".to_string(),
                    upload: "".to_string(),
                },
            },
        }
    }
}

impl Settings {
    /// # Errors
    ///
    /// Will return `Err` if it is not possible to build or deserialize the configuration file.
    pub fn load() -> Result<Self, ConfigError> {
        let config_path = std::env::var(FRONTEND_CONFIG_PATH_ENV)
            .unwrap_or_else(|_| String::from(FRONTEND_DEFAULT_CONFIG_FILE_PATH));

        let config = Config::builder()
            .add_source(ConfigFile::with_name(config_path.as_str()))
            // Allow environment variables to set/override config parsing '__' as '.'
            // Keep '_' is needed due to attribute names
            .add_source(Environment::with_prefix("FRONTEND").separator("__"))
            .build()?;

        config.try_deserialize()
    }
}
