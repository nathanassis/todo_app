use std::{env, process};

struct Config {
    function: String,
    params: Option<Vec<String>>,
}

impl Config {
    fn build(args: &Vec<String>) -> Result<Config, &'static str> {
        if args.len() == 1 {
            return Ok(Config {
                function: String::from("list"),
                params: None,
            });
        }

        let mut args_clone = args.clone();
        args_clone.remove(0);
        Ok(Config {
            function: args_clone.remove(0),
            params: Some(args_clone),
        })
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::build(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {err}");
        process::exit(1);
    });
    println!("function: {}", config.function);
    println!("params: {:?}", config.params);
}


/*
- NEXT STEPS:
- create a match to Config.function
- depending on this value take the right action
*/