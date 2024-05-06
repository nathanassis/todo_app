use std::error::Error;

pub struct Config {
    function: String,
    params: Option<Vec<String>>,
}

impl Config {
    pub fn build(args: &Vec<String>) -> Result<Config, &'static str> {
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

    pub fn add(&self) {}
    pub fn remove(&self) {}
    pub fn done(&self) {}
    pub fn list(&self) {}
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    match config.function.as_str() {
        "add" => config.add(),
        "remove" => config.remove(),
        "done" => config.done(),
        "list" => config.list(),
        _ => unreachable!(),
    };

    Ok(())
}
