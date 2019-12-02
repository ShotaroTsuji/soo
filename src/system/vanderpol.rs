use ndarray::*;
use eom::traits::*;

#[derive(Debug, Clone, Copy)]
pub struct VanDerPol {
    pub mu: f64,
}

impl Default for VanDerPol {
    fn default() -> Self {
        VanDerPol {
            mu: 1.0,
        }
    }
}

impl ModelSpec for VanDerPol {
    type Scalar = f64;
    type Dim = Ix1;

    fn model_size(&self) -> usize {
        2
    }
}

impl Explicit for VanDerPol {
    fn rhs<'a, S>(&mut self, v: &'a mut ArrayBase<S, Ix1>) -> &'a mut ArrayBase<S, Ix1>
    where
        S: DataMut<Elem = f64>,
    {
        let x = v[0];
        let y = v[1];

        v[0] = y;
        v[1] = self.mu*(1.0 - x*x)*y - x;
 
        v
    }
}

use crate::SystemFromConfig;
use crate::config;

impl SystemFromConfig for VanDerPol {
    fn system_from_config(conf: &config::Config) -> Self {
        assert_eq!(&conf.system.name, "vanderpol");

        let mu = crate::lookup_parameter!("mu", &conf);

        VanDerPol {
            mu: mu,
        }
    }
}
