use ndarray::*;
use eom::traits::*;

#[derive(Debug, Clone, Copy)]
pub struct Harmonic {
    pub omega: f64,
    pub alpha: f64,
}

impl Default for Harmonic {
    fn default() -> Self {
        Harmonic {
            omega: 1.0,
            alpha: 0.0,
        }
    }
}

impl ModelSpec for Harmonic {
    type Scalar = f64;
    type Dim = Ix1;

    fn model_size(&self) -> usize {
        2
    }
}

impl Explicit for Harmonic {
    fn rhs<'a, S>(&mut self, v: &'a mut ArrayBase<S, Ix1>) -> &'a mut ArrayBase<S, Ix1>
    where
        S: DataMut<Elem = f64>,
    {
        let x = v[0];
        let y = v[1];

        v[0] = self.omega * y;
        v[1] = -self.omega * x - self.alpha * y;

        v
    }
}

use crate::SystemFromConfig;
use crate::config;

impl SystemFromConfig for Harmonic {
    fn system_from_config(conf: &config::Config) -> Self {
        assert_eq!(&conf.system.name, "harmonic");

        let omega = crate::lookup_parameter!("omega", &conf);
        let alpha = crate::try_lookup_parameter!("alpha", &conf).unwrap_or(0.0);

        Harmonic {
            omega: omega,
            alpha: alpha,
        }
    }
}
