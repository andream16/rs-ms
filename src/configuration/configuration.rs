extern crate config;
extern crate envy;

use std::fmt;
use std::error;
use std::env;

/// The `Environment` type contains information used to startup the server
#[derive(Deserialize,Clone)]
pub struct Environment {
    #[serde(rename="hostname")]
    pub hostname: String,
    #[serde(rename="port")]
    pub port: String,
}

#[derive(Debug)]
pub enum EnvironmentErr {
    VarsErr(envy::Error),
    ConfigErr(config::ConfigError)
}

impl fmt::Display for EnvironmentErr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            EnvironmentErr::VarsErr(ref e) => e.fmt(f),
            EnvironmentErr::ConfigErr(ref e) => e.fmt(f),
        }
    }
}

impl error::Error for EnvironmentErr {
    fn description(&self) -> &str {
        match *self {
            EnvironmentErr::VarsErr(ref e) => e.description(),
            EnvironmentErr::ConfigErr(ref e) => e.description(),
        }
    }

    fn cause(&self) -> Option<&error::Error> {
        match *self {
            EnvironmentErr::VarsErr(ref e) => Some(e),
            EnvironmentErr::ConfigErr(ref e) => Some(e),
        }
    }
}

pub fn get() -> Result<Environment, EnvironmentErr> {

    match load_from_env() {
        Ok(r) => match valid(&r) {
            true => {
                Ok(r.clone())
            },
            _ => {
                load_from_config()
            },
        },
        Err(_) => {
            load_from_config()
        },
    }

}

fn valid(ev: &Environment) -> bool {
    return !ev.hostname.is_empty() && !ev.port.is_empty()
}

fn load_from_env() -> Result<Environment, EnvironmentErr> {
    match envy::from_env::<Environment>() {
        Ok(v) => Ok(v),
        Err(e) => Err(EnvironmentErr::VarsErr(e))
    }
}

/// Returns a new `Environment`
///
/// # Safe
///
/// Initializes a new `Environment` from environment variables.
/// If no environment variables are found, it initializes it from `default-config.yaml`.
fn load_from_config() -> Result<Environment, EnvironmentErr> {

    let file_name = "default-env";

    let mut settings = config::Config::default();

    if let Err(e) = settings.merge(config::File::with_name(file_name)) {
        return Err(EnvironmentErr::ConfigErr(e));
    }

    match settings.try_into::<Environment>() {
        Ok(r) => Ok(r),
        Err(e) => Err(EnvironmentErr::ConfigErr(e)),
    }

}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn it_loads_from_config() {

        let hostname = "127.0.0.1".to_string();
        let port = "8080".to_string();

        let ev = load_from_config();

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

    #[test]
    fn it_loads_from_env() {

        let hostname = "127.0.0.1".to_string();
        let port = "8080".to_string();

        env::set_var("hostname", hostname);
        env::set_var("port", port);

        let ev = load_from_env();

        match ev {
            Ok(v) => {
                assert_eq!(v.hostname, v.hostname);
                assert_eq!(v.port, v.port);
            }
            Err(_) => {
                assert!(true)
            }
        }

    }

}
