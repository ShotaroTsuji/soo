#[macro_use]
pub mod config;
pub mod system;

pub trait SystemFromConfig {
    fn system_from_config(conf: &config::Config) -> Self;
}
