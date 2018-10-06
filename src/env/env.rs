extern crate config;

/// The `Environment` type contains information used to startup the server
#[derive(Deserialize)]
pub struct Environment {
    #[serde(rename="hostname")]
    pub hostname: String,
    #[serde(rename="port")]
    pub port: String,
}

/// Returns a new `Environment`
///
/// # Safe
///
/// Initializes a new `Environment` from environment variables.
/// If no environment variables are found, it initializes it from `default-config.yaml`.
pub fn get() -> Result<Environment, config::ConfigError> {

    let file_name = "default-env";

    let mut settings = config::Config::default();

    if let Err(e) = settings.merge(config::File::with_name(file_name)) {
        return Err(e)
    }

    return settings.try_into::<Environment>();

}
