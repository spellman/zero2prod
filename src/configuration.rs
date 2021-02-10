// TODO: Should probably introduce types instead of using primitives.

#[derive(serde::Deserialize)]
pub struct DatabaseSettings {
    pub username: String,
    pub password: String,
    pub port: u16,
    pub host: String,
    pub database_name: String,
}

#[derive(serde::Deserialize)]
pub struct Settings {
    pub database: DatabaseSettings,
    pub application_port: u16,
}

pub fn get_configuration() -> Result<Settings, config::ConfigError> {
    // Initialize configuration reader.
    let mut settings = config::Config::default();

    // Add configuration values from a file named "configuration". The config crate will look for
    // any top-level file with an extension that it knows how to parse: yaml, json, some others.
    settings.merge(config::File::with_name("configuration"))?;

    // Try to convert the read configuration values into our Settings type.
    settings.try_into()
}
