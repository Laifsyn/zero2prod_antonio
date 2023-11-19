//! src/configuration.rs
const CONFIGURATION_PATH: &str = "configuration.yaml";
#[derive(serde::Deserialize, Debug)]
pub struct Settings {
    pub database: DatabaseSettings,
    pub application_port: u16,
}

#[derive(serde::Deserialize, Debug)]
pub struct DatabaseSettings {
    pub username: String,
    pub password: String,
    pub port: u16,
    pub host: String,
    pub database_name: String,
}

pub fn get_configuration() -> Result<Settings, config::ConfigError> {
    // Initialise our configuration reader
    let settings = config::Config::builder()
        // Add configuration values from a file named `configuration.yaml`.
        .add_source(config::File::new(
            CONFIGURATION_PATH,
            config::FileFormat::Yaml,
        ))
        .build()?;
    // Try to convert the configuration values it read into
    // our Settings type
    let ret = settings.try_deserialize::<Settings>();
    #[cfg(debug_assertions)]
    if let Ok(ret) = &ret {
        println!("Configuration Deserialization result: {ret:?}");
    }
    ret
}

impl DatabaseSettings {
    pub fn connection_string(&self) -> String {
        let ret = format!(
            "postgres://{}:{}@{}:{}/{}",
            self.username, self.password, self.host, self.port, self.database_name
        );
        #[cfg(debug_assertions)]
        println!("Database Connection Link generated: {ret:?}");
        ret
    }
}