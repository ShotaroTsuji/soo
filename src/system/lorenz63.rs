use crate::SystemFromConfig;
use crate::config;
use eom::ode::lorenz63::Lorenz63;

impl SystemFromConfig for Lorenz63 {
    fn system_from_config(conf: &config::Config) -> Self {
        assert_eq!(&conf.system.name, "lorenz63");

        Lorenz63 {
            p: crate::lookup_parameter!("p", &conf),
            r: crate::lookup_parameter!("r", &conf),
            b: crate::lookup_parameter!("b", &conf),
        }
    }
}
