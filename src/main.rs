use std::io::Read;
use soo::SystemFromConfig;
use soo::system::harmonic::Harmonic;
use soo::system::vanderpol::VanDerPol;
use eom::traits::Scheme;
use eom::ode::roessler::Roessler;
use eom::ode::lorenz63::Lorenz63;
use rand::distributions::Uniform;
use rand_distr::{Distribution, Normal};

macro_rules! print_dist_ts {
    ($dist:expr, $range_start:expr, $range_end:expr) => {
        let mut t = 0;
        let mut rng = rand::thread_rng();
        loop {
            if let Some(end) = $range_end {
                if t > end { break; }
            }
            let v = $dist.sample(&mut rng);
            println!("{:.8}", v);
            t += 1;
        }
    };
}

macro_rules! print_ode_ts {
    ($solver:ty, $ode:expr, $init:expr, $step_size:expr, $range_start:expr, $range_end:expr) => {
        let mut t = 1;
        let mut teo = <$solver>::new($ode, $step_size);
        let mut ts = eom::adaptor::time_series(ndarray::arr1($init), &mut teo);

        print!("{:.8}", 0.0);
        for x in $init.clone().iter() {
            print!(" {:.8}", x);
        }
        println!("");

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
    ($scheme:ty, $ode:expr, $conf_generate:expr) => {
        print_ode_ts!($scheme, $ode,
                      $conf_generate.init.as_ref().unwrap(),
                      $conf_generate.step_size.unwrap(),
                      $conf_generate.step_range.0,
                      $conf_generate.step_range.1);
    };
}

fn main() {
    let mut handle = std::io::stdin();
    let mut conf = String::new();
    handle.read_to_string(&mut conf).unwrap();

    let conf = soo::config::read_config(&conf);

    match conf.system.name.as_str() {
        "harmonic" => {
            let ode = Harmonic::system_from_config(&conf);
            match conf.generate.solver.as_str() {
                "RK4" => { ode_match_arm!(eom::explicit::RK4<Harmonic>, ode, conf.generate); },
                "euler" => { ode_match_arm!(eom::explicit::Euler<Harmonic>, ode, conf.generate); },
                _ => panic!("unknown solver"),
            }
        },
        "vanderpol" => {
            let ode = VanDerPol::system_from_config(&conf);
            match conf.generate.solver.as_str() {
                "RK4" => { ode_match_arm!(eom::explicit::RK4<VanDerPol>, ode, conf.generate); },
                "euler" => { ode_match_arm!(eom::explicit::Euler<VanDerPol>, ode, conf.generate); },
                _ => panic!("unknown solver"),
            }
        },
        "roessler" => {
            let ode = Roessler::system_from_config(&conf);
            match conf.generate.solver.as_str() {
                "RK4" => { ode_match_arm!(eom::explicit::RK4<Roessler>, ode, conf.generate); },
                "euler" => { ode_match_arm!(eom::explicit::Euler<Roessler>, ode, conf.generate); },
                _ => panic!("unknown solver"),
            }
        },
        "lorenz63" => {
            let ode = Lorenz63::system_from_config(&conf);
            match conf.generate.solver.as_str() {
                "RK4" => { ode_match_arm!(eom::explicit::RK4<Lorenz63>, ode, conf.generate); },
                "euler" => { ode_match_arm!(eom::explicit::Euler<Lorenz63>, ode, conf.generate); },
                _ => panic!("unknown solver"),
            }
        },
        "normal" => {
            let dist = Normal::system_from_config(&conf);
            print_dist_ts!(dist, conf.generate.step_range.0, conf.generate.step_range.1);
        },
        "uniform" => {
            let dist = Uniform::system_from_config(&conf);
            print_dist_ts!(dist, conf.generate.step_range.0, conf.generate.step_range.1);
        },
        _ => panic!("unknown system"),
    }
}
