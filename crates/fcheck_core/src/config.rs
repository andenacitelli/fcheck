use envconfig::Envconfig;

#[derive(Envconfig)]
pub struct Config {
    #[envconfig(from = "RUST_LOG")]
    pub rust_log: String,
}

/// Realias of get_environment() called in main() to better connotate that we went through an actual validation step
pub fn validate() {
    get_environment();
}

pub fn get_environment() -> Config {
    Config::init_from_env().unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::env;
    fn init() {
        let _ = env_logger::builder().is_test(true).try_init();
    }

    #[test]
    fn test_get_environment() {
        init();
        env::set_var("RUST_LOG", "info");
        let _config = get_environment();
    }
}
