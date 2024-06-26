mod todo_list;
use todo_list::TodoList;

pub struct Config {
    function: String,
    params: Vec<String>,
}

impl Config {
    pub fn build(args: &Vec<String>) -> Result<Config, &'static str> {
        if args.len() == 1 {
            return Ok(Config {
                function: String::from("list"),
                params: vec![],
            });
        }

        let mut args_clone = args.clone();
        args_clone.remove(0);
        Ok(Config {
            function: args_clone.remove(0),
            params: Self::parse_params(args_clone),
        })
    }

    fn parse_params(params: Vec<String>) -> Vec<String> {
        let mut param = String::new();
        let mut new_params: Vec<String> = Vec::new();

        for p in params {
            if p.ends_with(",") {
                param.push_str(&p.as_str()[..p.len() - 1]);
                new_params.push(param);
                param = String::new();
            } else {
                param.push_str((p + " ").as_str());
            }
        }
        param.pop();
        new_params.push(param);

        new_params
    }
}

pub fn run(config: Config) -> Result<(), &'static str> {
    let todo_list = match TodoList::build() {
        Ok(todo_list) => todo_list,
        Err(err) => return Err(err),
    };

    match config.function.as_str() {
        "add" => todo_list.add(&config.params),
        "remove" => todo_list.remove(&config.params),
        "list" => todo_list.list(),
        _ => unreachable!(),
    }
}
