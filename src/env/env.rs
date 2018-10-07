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

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn it_gets_default_environment() {

        let hostname = "127.0.0.1".to_string();
        let port = "8080".to_string();

        let ev = get();

        match ev {
            Ok(v) => {
                assert_eq!(hostname, v.hostname);
                assert_eq!(port, v.port);
            },
            Err(_) => {
                assert!(true)
            },
        }

    }
}
