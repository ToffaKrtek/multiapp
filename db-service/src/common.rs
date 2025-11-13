pub struct Config {
    pub host: String,
    pub password: String,
}

impl Config {
    pub fn new(host: String, password: String) -> Config {
        Config {
            host: host,
            password: password,
        }
    }
}
