use std::env;

use crate::common::Config;

const HOST_KEY: &str = "HOST_KEY";
const PASSWORD_KEY: &str = "PASSWORD_KEY";

pub fn config() -> Config {
    let host = match env::var(HOST_KEY) {
        Ok(val) => val,
        _ => "localhost".to_string(),
    };
    let password = match env::var(PASSWORD_KEY) {
        Ok(val) => val,
        _ => "root".to_string(),
    };
    return Config::new(host, password);
}
