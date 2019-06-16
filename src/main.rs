use std::io::Read;
use soo::SystemFromConfig;
use eom::traits::Scheme;
use eom::ode::roessler::Roessler;
use eom::ode::lorenz63::Lorenz63;

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
                print!("{:.8}", time);
                for x in v.iter() {
                    print!(" {:.8}", x);
                }
                println!("");
            }
            t += 1;
        }
    };
}

macro_rules! ode_match_arm {
    ($scheme:ty, $ode:expr, $conf_solver:expr) => {
        print_ode_ts!($scheme, $ode,
                      &$conf_solver.init.unwrap(),
                      $conf_solver.step_size.unwrap(),
                      $conf_solver.step_range.0,
                      $conf_solver.step_range.1);
    };
}

fn main() {
    let mut handle = std::io::stdin();
    let mut conf = String::new();
    handle.read_to_string(&mut conf).unwrap();

    let conf = soo::config::read_config(&conf);
    println!("{:#?}", conf);

    match conf.system.name.as_str() {
        "roessler" => {
            let ode = Roessler::system_from_config(&conf);
            match conf.solver.name.as_str() {
                "RK4" => { ode_match_arm!(eom::explicit::RK4<Roessler>, ode, conf.solver); },
                _ => panic!("unknown solver"),
            }
        },
        "lorenz63" => {
            let ode = Lorenz63::system_from_config(&conf);
            match conf.solver.name.as_str() {
                "RK4" => { ode_match_arm!(eom::explicit::RK4<Lorenz63>, ode, conf.solver); },
                _ => panic!("unknown solver"),
            }
        },
        _ => panic!("unknown system"),
    }
}
