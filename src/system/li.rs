use ndarray::*;
use eom::traits::*;

#[derive(Debug, Clone, Copy)]
pub struct Li {
    pub a: f64,
    pub c: f64,
    pub d: f64,
    pub e: f64,
    pub k: f64,
    pub f: f64,
}

impl Default for Li {
    fn default() -> Self {
        Li {
            a: 40.0,
            c: 11.0/6.0,
            d: 0.16,
            e: 0.65,
            k: 55.0,
            f: 20.0,
        }
    }
}

impl ModelSpec for Li {
    type Scalar = f64;
    type Dim = Ix1;

    fn model_size(&self) -> usize {
        3
    }
}

impl Explicit for Li {
    fn rhs<'a, S>(&mut self, v: &'a mut ArrayBase<S, Ix1>) -> &'a mut ArrayBase<S, Ix1>
    where
        S: DataMut<Elem = f64>,
    {
        let x = v[0];
        let y = v[1];
        let z = v[2];

        v[0] = self.a*(y - x) + self.d*x*z;
        v[1] = self.k*x + self.f*y - x*z;
        v[2] = self.c*z + x*y - self.e*x*x;
 
        v
    }
}

use crate::SystemFromConfig;
use crate::config;

impl SystemFromConfig for Li {
    fn system_from_config(conf: &config::Config) -> Self {
        assert_eq!(&conf.system.name, "li");

        Default::default()
    }
}
