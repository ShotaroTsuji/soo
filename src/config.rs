use std::convert::TryInto;

#[derive(Debug, Clone)]
pub struct Config {
    system: System,
    solver: Solver,
}

#[derive(Debug, Clone)]
pub struct System {
    name: String,
    parameter: toml::value::Table,
}

#[derive(Debug, Clone)]
pub struct Solver {
    name: String,
    step_range: (usize, Option<usize>),
    step_size: Option<f64>,
    init: Option<Vec<f64>>,
}

pub fn read_config(conf: &str) -> Config {
    let conf = conf.parse::<toml::Value>()
        .expect("config file must be written in TOML");

    let system = {
        let system = conf.get("system")
            .expect("table [system] is missing");

        let name = system.get("name")
            .expect("system name is missing")
            .as_str()
            .expect("system name must be a string")
            .to_string();
        let parameter = system.get("parameter")
            .expect("system parameter is missing")
            .as_table()
            .expect("system parameter must be a table")
            .clone();

        System {
            name: name,
            parameter: parameter,
        }
    };

    let solver = {
        let solver = conf.get("solver")
            .expect("table [solver] is missing");

        let name = solver.get("name")
            .expect("solver name is missing")
            .as_str()
            .expect("solver name must be a string")
            .to_string();

        let step = solver.get("step")
            .expect("solver.step is missing")
            .as_table()
            .expect("solver.step must be a table");

        let step_range = step.get("range")
            .expect("solver.step.range is missing")
            .as_table()
            .expect("solver.step.range must be a table");

        let step_range_start: usize =
            step_range.get("start")
            .expect("range start is missing")
            .as_integer()
            .expect("range start must be integer")
            .try_into() // TryInto of i64
            .expect("range start must be unsigned integer");

        let step_range_end: Option<usize> =
            step_range.get("end")
            .map(|end| end.as_integer()
                 .expect("range end must be integer")
                 .try_into() // TryInto of i64
                 .expect("range end must be unsigned integer"));

        let step_size = step.get("size")
            .map(|size| size.as_float()
                 .expect("step size must be float"));

        let init = solver.get("init")
            .map(|init| init.as_array().expect("init must be an array"))
            .map(|init| init.iter()
                 .map(|value| value.as_float().expect("init value must be float"))
                 .collect::<Vec<f64>>());

        Solver {
            name: name,
            step_range: (step_range_start, step_range_end),
            step_size: step_size,
            init: init,
        }
    };

    Config {
        system: system,
        solver: solver,
    }
}
