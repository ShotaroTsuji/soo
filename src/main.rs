use std::io::Read;
use soo::SystemFromConfig;
use eom::traits::Scheme;
use eom::ode::roessler::Roessler;

fn main() {
    let mut handle = std::io::stdin();
    let mut conf = String::new();
    handle.read_to_string(&mut conf).unwrap();

    let conf = soo::config::read_config(&conf);
    println!("{:#?}", conf);

    let ode = Roessler::system_from_config(&conf);
    println!("{:?}", ode);

    let mut t = 0;
    let mut teo = eom::explicit::RK4::new(ode, conf.solver.step_size.unwrap());
    let mut ts = eom::adaptor::time_series(ndarray::arr1(&conf.solver.init.unwrap()), &mut teo);
    loop {
        if t > conf.solver.step_range.1.unwrap() {
            break;
        }
        let v = ts.next().unwrap();
        let time = t as f64 * conf.solver.step_size.unwrap();
        println!("{:.6} {:.6} {:.6} {:.6}", time, v[0], v[1], v[2]);
        t += 1;
    }
}
