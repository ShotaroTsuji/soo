pub mod config;

pub trait SystemFromConfig {
    fn system_from_config(conf: &config::Config) -> Self;
}
