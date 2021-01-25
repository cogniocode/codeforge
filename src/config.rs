use config::Config;

pub fn init() -> Config {
    let mut config = Config::default();

    config
        .merge(config::File::with_name("app")).unwrap()
        .merge(config::Environment::with_prefix("CODEFORGE")).unwrap();

    config
}