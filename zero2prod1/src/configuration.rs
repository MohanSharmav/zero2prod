#[derive(serde::Deserialize)]
pub struct Settings {
    pub database: databaseSettings,
    pub application_port: u16,
}
#[derive(serde::Deserialize)]
pub struct databaseSettings {
    pub username: String,
    pub password: String,
    pub host: String,
    pub port: u16,
    pub database_name: String,
}

pub fn get_configuration()->Result<Settings,config::ConfigError>
{
let settings= config::Config::builder()
    .add_source(config::File::with_name("configuration.yaml", config::FileFormat::Yaml)
    )
    .build()?;

    settings.try_deserialize::<Settings>()
}



