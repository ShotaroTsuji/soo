use crate::SystemFromConfig;
use crate::config;
use rand::distributions::Uniform;

impl SystemFromConfig for Uniform<f64> {
    fn system_from_config(conf: &config::Config) -> Self {
        assert_eq!(&conf.system.name, "uniform");

        let low = crate::lookup_parameter!("low", &conf);
        let high = crate::lookup_parameter!("high", &conf);

        Uniform::new_inclusive(low, high)
    }
}
