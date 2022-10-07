use std::env;

use dotenv::{dotenv, from_filename};

use super::env_variables::Configuration;

pub fn init_env_variables() -> Configuration {
    let env = match env::var_os("ENV") {
        Some(value) => value.into_string().unwrap(),
        None => "LOCAL".to_string(),
    };

    let env_file = from_filename(format!(".env.{}", env.to_lowercase())).ok();

    match env_file {
        Some(_) => env_file,
        _ => dotenv().ok(),
    };
    match envy::from_env::<Configuration>() {
        // if we could load the config using the existing env variables - use that
        Ok(config) => config,
        Err(error) => panic!("{:#?}", error),
    }
}

pub fn env_variables() -> Configuration {
    match envy::from_env::<Configuration>() {
        // if we could load the config using the existing env variables - use that
        Ok(config) => config,
        Err(error) => panic!("{:#?}", error),
    }
}
