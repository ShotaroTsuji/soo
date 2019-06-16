use std::io::Read;
use soo::SystemFromConfig;
use eom::traits::Scheme;
use eom::ode::roessler::Roessler;

macro_rules! print_ode_ts {
    ($solver:ty, $ode:expr, $init:expr, $step_size:expr, $range_start:expr, $range_end:expr) => {
        let mut t = 0;
        let mut teo = <$solver>::new($ode, $step_size);
        let mut ts = eom::adaptor::time_series(ndarray::arr1($init), &mut teo);
        loop {
            if let Some(end) = $range_end {
                if t > end { break; }
            }
            let v = ts.next().unwrap();
            if t >= $range_start {
                let time = t as f64 * $step_size;
                println!("{:.6} {:.6} {:.6} {:.6}", time, v[0], v[1], v[2]);
            }
            t += 1;
        }
    };
}

fn main() {
    let mut handle = std::io::stdin();
    let mut conf = String::new();
    handle.read_to_string(&mut conf).unwrap();

    let conf = soo::config::read_config(&conf);
    println!("{:#?}", conf);

    let ode = Roessler::system_from_config(&conf);
    println!("{:?}", ode);

    print_ode_ts!(
        eom::explicit::RK4<Roessler>,
        ode,
        &conf.solver.init.unwrap(),
        conf.solver.step_size.unwrap(),
        conf.solver.step_range.0,
        conf.solver.step_range.1);
}
