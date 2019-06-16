use crate::SystemFromConfig;
use crate::config;
use eom::ode::roessler::Roessler;

impl SystemFromConfig for Roessler {
    fn system_from_config(conf: &config::Config) -> Self {
        assert_eq!(&conf.system.name, "roessler");

        let a = crate::lookup_parameter!("a", &conf);
        let b = crate::lookup_parameter!("b", &conf);
        let c = crate::lookup_parameter!("c", &conf);

        Roessler {
            a: a,
            b: b,
            c: c,
        }
    }
}
