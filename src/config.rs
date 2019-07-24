use std::convert::TryInto;

#[macro_export]
macro_rules! lookup_parameter {
    ($name:expr, $conf:expr) => {
        $conf.system.parameter.get($name)
            .expect(concat!("parameter ", $name, " is missing"))
            .as_float()
            .expect(concat!("parameter ", $name, " must be a float"))
    };
}

#[derive(Debug, Clone)]
pub struct Config {
    pub system: System,
    pub generate: Generate,
}

#[derive(Debug, Clone)]
pub struct System {
    pub name: String,
    pub parameter: toml::value::Table,
}

#[derive(Debug, Clone)]
pub struct Generate {
    pub solver: String,
    pub step_range: (usize, Option<usize>),
    pub step_size: Option<f64>,
    pub init: Option<Vec<f64>>,
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

    let generate = {
        let generate = conf.get("generate")
            .expect("table [generate] is missing");

        let solver = generate.get("solver")
            .expect("solver name is missing")
            .as_str()
            .expect("solver name must be a string")
            .to_string();

        let step = generate.get("step")
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

        let init = generate.get("init")
            .map(|init| init.as_array().expect("init must be an array"))
            .map(|init| init.iter()
                 .map(|value| value.as_float().expect("init value must be float"))
                 .collect::<Vec<f64>>());

        Generate {
            solver: solver,
            step_range: (step_range_start, step_range_end),
            step_size: step_size,
            init: init,
        }
    };

    Config {
        system: system,
        generate: generate,
    }
}
