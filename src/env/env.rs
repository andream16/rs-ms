extern crate config;

#[derive(Deserialize)]
pub struct Environment {
    #[serde(rename="hostname")]
    pub hostname: String,
    #[serde(rename="port")]
    pub port: String,
}

pub fn get() -> Result<Environment, config::ConfigError> {

    let mut settings = config::Config::default();

    settings
        .merge(config::File::with_name("default-env")).unwrap();

    return settings.try_into::<Environment>();

}
