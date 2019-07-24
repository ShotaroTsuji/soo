use crate::SystemFromConfig;
use crate::config;
use rand_distr::Normal;

impl SystemFromConfig for Normal<f64> {
    fn system_from_config(conf: &config::Config) -> Self {
        assert_eq!(&conf.system.name, "normal");

        let mean = crate::lookup_parameter!("mean", &conf);
        let std_dev = crate::lookup_parameter!("std_dev", &conf);

        Normal::new(mean, std_dev).unwrap()
    }
}
